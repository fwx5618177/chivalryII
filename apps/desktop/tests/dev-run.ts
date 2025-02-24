import { createClient, ClientConfig } from '../ts/index';

async function main() {
    const client = createClient();
    
    const config: ClientConfig = {
        width: 800,
        height: 600,
        title: "Test Window",
        vsync: true
    };

    try {
        await client.initialize(config);
        await client.run();
        await client.cleanup();
    } catch (error) {
        console.error('Error:', error);
    }
}

main().catch(console.error);

// Handle window close event
process.on('SIGINT', () => {
  console.log('Shutting down desktop client...');
  process.exit(0);
});
