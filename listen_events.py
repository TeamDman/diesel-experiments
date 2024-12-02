import asyncio
import os
from dotenv import load_dotenv
import asyncpg

async def listen_to_notifications():
    # Load environment variables from the .env file
    load_dotenv()
    database_url = os.getenv("DATABASE_URL")
    if not database_url:
        raise ValueError("DATABASE_URL must be set in the environment")

    # Connect to the PostgreSQL database
    conn = await asyncpg.connect(database_url)
    print("Connected to the database.")

    # Listen to the test_notifications channel
    await conn.add_listener('test_notifications', notification_handler)
    print("Listening to channel 'test_notifications'.")

    # Keep the connection alive
    try:
        await asyncio.Future()  # Run forever
    except asyncio.CancelledError:
        pass
    finally:
        # Clean up the connection when exiting
        await conn.remove_listener('test_notifications', notification_handler)
        await conn.close()
        print("Connection closed.")

# Handler for notifications
def notification_handler(connection, pid, channel, payload):
    print(f"Received notification on channel '{channel}': {payload}")

# Entry point
if __name__ == "__main__":
    try:
        asyncio.run(listen_to_notifications())
    except KeyboardInterrupt:
        print("\nExiting listener.")
