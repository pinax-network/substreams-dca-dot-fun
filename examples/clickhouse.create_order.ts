import { client } from './clickhouse.config.js';

// SELECT create_order
const query = `SELECT * FROM create_order LIMIT 3`;
const result = await client.query({ query, format: 'JSONEachRow' });
const json = await result.json();

console.log('SELECT create_order');
for (const row of json) {
    console.log(row);
}