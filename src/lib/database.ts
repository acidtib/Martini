import Database from '@tauri-apps/plugin-sql'

// Base interfaces
interface ModelAttributes {
  id?: number
  [key: string]: any
}

interface QueryOptions {
  where?: { [key: string]: any }
  limit?: number
  orderBy?: { column: string; direction: 'ASC' | 'DESC' }
}

type Constructor<M> = {
  new (attrs: any): M & { tableName: string }
  find(options?: QueryOptions): Promise<M[]>
  findOne(options?: QueryOptions): Promise<M | null>
}

// Base Model class
abstract class Model<T extends ModelAttributes> {
  protected static db: Database | null = null
  protected static readonly dbUrl = 'sqlite:app.db'
  
  abstract tableName: string
  attributes: T

  constructor(attributes: T) {
    this.attributes = attributes
  }

  protected static async getDb(): Promise<Database> {
    if (!this.db) {
      this.db = await Database.load(this.dbUrl)
    }
    return this.db
  }

  protected buildWhereClause(where: { [key: string]: any }): { query: string; params: any[] } {
    const conditions: string[] = []
    const params: any[] = []
    
    Object.entries(where).forEach(([key, value]) => {
      conditions.push(`${key} = ?`)
      params.push(value)
    })
    
    return {
      query: conditions.length ? ` WHERE ${conditions.join(' AND ')}` : '',
      params
    }
  }

  async save(): Promise<this> {
    const db = await (this.constructor as typeof Model).getDb()
    const { id, ...attrs } = this.attributes
    
    if (id) {
      // Update
      const setClauses = Object.keys(attrs).map(key => `${key} = ?`).join(', ')
      const values = [...Object.values(attrs), id]
      
      await db.execute(
        `UPDATE ${this.tableName} SET ${setClauses} WHERE id = ?`,
        values
      )
    } else {
      // Insert
      const columns = Object.keys(attrs).join(', ')
      const placeholders = Object.keys(attrs).map(() => '?').join(', ')
      const values = Object.values(attrs)
      
      const result = await db.execute(
        `INSERT INTO ${this.tableName} (${columns}) VALUES (${placeholders})`,
        values
      )
      this.attributes.id = result.lastInsertId
    }
    
    return this
  }

  async delete(): Promise<void> {
    if (!this.attributes.id) throw new Error('Cannot delete unsaved model')
    
    const db = await (this.constructor as typeof Model).getDb()
    await db.execute(
      `DELETE FROM ${this.tableName} WHERE id = ?`,
      [this.attributes.id]
    )
  }

  static async find<M extends Model<any>>(
    this: Constructor<M>,
    options: QueryOptions = {}
  ): Promise<M[]> {
    const db = await Model.getDb()
    const instance = new this({})
    
    let query = `SELECT * FROM ${instance.tableName}`
    const params: any[] = []

    if (options.where) {
      const whereClause = instance.buildWhereClause(options.where)
      query += whereClause.query
      params.push(...whereClause.params)
    }

    if (options.orderBy) {
      query += ` ORDER BY ${options.orderBy.column} ${options.orderBy.direction}`
    }

    if (options.limit) {
      query += ` LIMIT ${options.limit}`
    }

    const results = await db.select<Record<string, any>[]>(query, params)
    return results.map((result: Record<string, any>) => new this(result))
  }

  static async findOne<M extends Model<any>>(
    this: Constructor<M>,
    options: QueryOptions = {}
  ): Promise<M | null> {
    const results = await this.find({ ...options, limit: 1 })
    return results[0] || null
  }
}

// Screenshot Model
interface ScreenshotAttributes extends ModelAttributes {
  name: string
  image: string
  created_at: string
}

class Screenshot extends Model<ScreenshotAttributes> {
  tableName = 'screenshots'

  static async latest(): Promise<Screenshot | null> {
    return this.findOne({
      orderBy: { column: 'created_at', direction: 'DESC' }
    })
  }
}

// Export database instance and models
// Export database instance and models
export { Screenshot, Model, type ModelAttributes, type QueryOptions, type ScreenshotAttributes }
