import { client } from './clickhouse.config.js';

// SHOW TABLES
const result = await client.query({ query: 'SHOW TABLES', format: 'JSONEachRow' });
const json: { name: string }[] = await result.json();

console.log('SHOW TABLES');
for (const row of json) {
    console.log('-', row.name);
}
