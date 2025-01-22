import Database from '@tauri-apps/plugin-sql'

// Base interfaces
interface BaseModel {
  id?: number
  created_at?: string
  [key: string]: any
}

interface QueryOptions {
  where?: { [key: string]: any }
  limit?: number
  orderBy?: { column: string; direction: 'ASC' | 'DESC' }
}

// Base Model class
abstract class Model<T extends BaseModel> {
  protected static db: Database | null = null
  protected static readonly dbUrl = 'sqlite:app.db'
  
  abstract get tableName(): string
  protected get uniqueKeys(): string[] { return [] }
  
  protected constructor(protected attributes: T) {}

  protected static async getDb(): Promise<Database> {
    if (!this.db) {
      this.db = await Database.load(this.dbUrl)
    }
    return this.db
  }

  protected buildWhereClause(where: { [key: string]: any }): { query: string; values: any[] } {
    const conditions: string[] = []
    const values: any[] = []
    
    Object.entries(where).forEach(([key, value]) => {
      conditions.push(`${key} = ?`)
      values.push(value)
    })
    
    return {
      query: conditions.length ? ` WHERE ${conditions.join(' AND ')}` : '',
      values
    }
  }

  async save(): Promise<number> {
    try {
      const db = await Model.getDb()
      const { id, created_at, ...attrs } = this.attributes
      
      // Check for existing record based on unique keys
      if (this.uniqueKeys.length > 0) {
        const whereConditions: { [key: string]: any } = {}
        this.uniqueKeys.forEach(key => {
          if (key in attrs) {
            whereConditions[key] = attrs[key]
          }
        })
        
        if (Object.keys(whereConditions).length > 0) {
          const { query, values } = this.buildWhereClause(whereConditions)
          const existing = await db.select<{ id: number }[]>(
            `SELECT id FROM ${this.tableName}${query}`,
            values
          )
          
          if (existing.length > 0) {
            // Update existing record
            const setClauses = Object.keys(attrs)
              .filter(key => !this.uniqueKeys.includes(key))
              .map(key => `${key} = ?`)
              .join(', ')
            const values = [
              ...Object.entries(attrs)
                .filter(([key]) => !this.uniqueKeys.includes(key))
                .map(([, value]) => value),
              existing[0].id
            ]
            
            await db.execute(
              `UPDATE ${this.tableName} SET ${setClauses} WHERE id = ?`,
              values
            )
            this.attributes.id = existing[0].id
            return existing[0].id
          }
        }
      }
      
      // Insert new record if no existing record found
      const columns = Object.keys(attrs)
      const placeholders = columns.map(() => '?').join(', ')
      const values = Object.values(attrs)
      
      const newId = (await db.execute(
        `INSERT INTO ${this.tableName} (${columns.join(', ')}) VALUES (${placeholders})`,
        values
      )).lastInsertId
      if (typeof newId === 'number') {
        this.attributes.id = newId
        return newId
      }
      throw new Error('Failed to get new ID after insert')
    } catch (error) {
      console.error(`Error saving ${this.tableName}:`, error)
      throw new Error(`Failed to save ${this.tableName}`)
    }
  }

  async delete(): Promise<boolean> {
    try {
      if (!this.attributes.id) {
        throw new Error('Cannot delete unsaved model')
      }
      
      const db = await Model.getDb()
      const result = await db.execute(
        `DELETE FROM ${this.tableName} WHERE id = ?`,
        [this.attributes.id]
      )
      return result.rowsAffected > 0
    } catch (error) {
      console.error(`Error deleting ${this.tableName}:`, error)
      throw new Error(`Failed to delete ${this.tableName}`)
    }
  }

  static async findById<M extends Model<any>>(
    this: new (attrs: any) => M,
    id: number
  ): Promise<M | null> {
    try {
      const instance = new this({})
      const db = await Model.getDb()
      const results = await db.select<{ id: number; [key: string]: any }>(
        `SELECT * FROM ${instance.tableName} WHERE id = ?`,
        [id]
      )
      return results[0] ? new this(results[0]) : null
    } catch (error) {
      console.error(`Error finding ${this.name} by ID:`, error)
      throw new Error(`Failed to find ${this.name}`)
    }
  }

  static async findAll<M extends Model<any>>(
    this: new (attrs: any) => M,
    options: QueryOptions = {}
  ): Promise<M[]> {
    try {
      const instance = new this({})
      const db = await Model.getDb()
      
      let query = `SELECT * FROM ${instance.tableName}`
      let values: any[] = []

      if (options.where) {
        const whereClause = instance.buildWhereClause(options.where)
        query += whereClause.query
        values = whereClause.values
      }

      if (options.orderBy) {
        query += ` ORDER BY ${options.orderBy.column} ${options.orderBy.direction}`
      }

      if (options.limit) {
        query += ` LIMIT ${options.limit}`
      }

      const results = await db.select<{ id: number; [key: string]: any }>(query, values)
      return results.map((result: { id: number; [key: string]: any }) => new this(result))
    } catch (error) {
      console.error(`Error finding all ${this.name}:`, error)
      throw new Error(`Failed to find ${this.name} records`)
    }
  }

  // Add public method to get attributes
  public getAttributes(): T {
    return this.attributes;
  }
}

// Screenshot Model implementation
interface Screenshot extends BaseModel {
  id: number;
  name: string;
  image: string;
  recognized: boolean;
  ocr: boolean;
  created_at: string;
}

export class Screenshots extends Model<Screenshot> {
  get tableName(): string {
    return 'screenshots'
  }

  constructor(attributes: Partial<Screenshot> = {}) {
    super(attributes as Screenshot)
  }
}

// Settings Model implementation
interface Setting extends BaseModel {
  key: string
  value: string
}

export class Settings extends Model<Setting> {
  get tableName(): string {
    return 'settings'
  }

  protected get uniqueKeys(): string[] {
    return ['key']
  }

  constructor(attributes: Partial<Setting> = {}) {
    super(attributes as Setting)
  }
}
