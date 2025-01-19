// import { db } from '../lib/database'
// 
// // Add a screenshot
// await db.addScreenshot('name', 'base64image')
//
// // Get latest screenshot
// const latest = await db.getLatestScreenshot()
//
// // Update a screenshot
// await db.updateScreenshot(1, { name: 'new name' })
//
// // Delete a screenshot
// await db.deleteScreenshot(1)


import Database from '@tauri-apps/plugin-sql'

export interface Screenshot {
  name: string
  image: string
  created_at: string
}

class DatabaseClient {
  private static instance: DatabaseClient
  private db: Database | null = null
  private readonly dbUrl = 'sqlite:app.db'

  private constructor() {}

  public static getInstance(): DatabaseClient {
    if (!DatabaseClient.instance) {
      DatabaseClient.instance = new DatabaseClient()
    }
    return DatabaseClient.instance
  }

  private async ensureConnection(): Promise<Database> {
    if (!this.db) {
      this.db = await Database.load(this.dbUrl)
    }
    return this.db
  }

  // Create a new screenshot
  async addScreenshot(name: string, image: string): Promise<Screenshot> {
    const db = await this.ensureConnection()
    const created_at = new Date().toISOString()
    
    // This is ok for now since we only care about 1 screenshot per session
    // the summary screen
    // Delete all previous screenshots
    await db.execute('DELETE FROM screenshots')
    
    // Insert the new screenshot    
    await db.execute(
      'INSERT INTO screenshots (name, image, created_at) VALUES ($1, $2, $3)',
      [name, image, created_at]
    )
    
    const result = await db.select<Screenshot[]>(
      'SELECT * FROM screenshots ORDER BY id DESC LIMIT 1'
    )
    
    if (!result.length) {
      throw new Error('Failed to create screenshot')
    }
    
    return result[0]
  }

  // Get a screenshot by ID
  async getScreenshot(id: number): Promise<Screenshot | null> {
    const db = await this.ensureConnection()
    const result = await db.select<Screenshot[]>(
      'SELECT * FROM screenshots WHERE id = $1',
      [id]
    )
    return result.length > 0 ? result[0] : null
  }

  // Get latest screenshot
  async getLatestScreenshot(): Promise<Screenshot | null> {
    const db = await this.ensureConnection()
    const result = await db.select<Screenshot[]>(
      'SELECT * FROM screenshots ORDER BY created_at DESC LIMIT 1'
    )
    return result.length > 0 ? result[0] : null
  }

  // Get all screenshots
  async getAllScreenshots(): Promise<Screenshot[]> {
    const db = await this.ensureConnection()
    const result = await db.select<Screenshot[]>(
      'SELECT * FROM screenshots ORDER BY created_at DESC'
    )
    return result
  }

  // Update a screenshot
  async updateScreenshot(id: number, data: Partial<Screenshot>): Promise<boolean> {
    const db = await this.ensureConnection()
    const updates: string[] = []
    const values: any[] = []
    let paramCount = 1

    Object.entries(data).forEach(([key, value]) => {
      if (key !== 'id' && key !== 'created_at') {
        updates.push(`${key} = $${paramCount}`)
        values.push(value)
        paramCount++
      }
    })

    if (updates.length === 0) return false

    values.push(id)
    const query = `UPDATE screenshots SET ${updates.join(', ')} WHERE id = $${paramCount}`
    
    const result = await db.execute(query, values)
    return result.rowsAffected > 0
  }

  // Delete a screenshot
  async deleteScreenshot(id: number): Promise<boolean> {
    const db = await this.ensureConnection()
    const result = await db.execute(
      'DELETE FROM screenshots WHERE id = $1',
      [id]
    )
    return result.rowsAffected > 0
  }
}

export const db = DatabaseClient.getInstance()
