import { createClient } from '@clickhouse/client-web'
import 'dotenv/config';

const database = 'base-sepolia:dca-dot-fun@v0.1.0';
if (!process.env.CLICKHOUSE_PASSWORD) {
    console.error('Please set the CLICKHOUSE_PASSWORD environment variable.');
    process.exit(1);
}
export const client = createClient({
    username: 'dca_user',
    password: process.env.CLICKHOUSE_PASSWORD,
    url: 'https://ch.token-api.service.dev.pinax.network',
    database
})
