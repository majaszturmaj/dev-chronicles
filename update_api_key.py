#!/usr/bin/env python3
"""
Script to update OpenAI API key in DevChronicle database
"""

import sqlite3
import os
from pathlib import Path
api_key = os.getenv("OPENAI_API_KEY")

# New API key
NEW_API_KEY = api_key
OPENAI_URL = "https://api.openai.com/v1"
MODEL_NAME = "gpt-4o-mini"

def find_database():
    """Find the DevChronicle database file."""
    possible_locations = [
        Path.home() / ".local" / "share" / "com.devchronicle.app" / "activity_logs.db",
        Path.home() / ".local" / "share" / "dev-chronicles" / "activity_logs.db",
    ]
    
    for db_path in possible_locations:
        if db_path.exists():
            return db_path
    
    # Try to find it by searching
    for location in [Path.home() / ".local" / "share"]:
        if location.exists():
            for item in location.iterdir():
                if item.is_dir():
                    db_file = item / "activity_logs.db"
                    if db_file.exists():
                        return db_file
    
    return None

def update_api_key(db_path: Path):
    """Update the API key in the database."""
    print(f"Connecting to database: {db_path}")
    
    conn = sqlite3.connect(str(db_path))
    cursor = conn.cursor()
    
    try:
        # Check if ai_settings table exists
        cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='ai_settings'")
        if not cursor.fetchone():
            print("Creating ai_settings table...")
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS ai_settings (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    provider_url TEXT NOT NULL,
                    api_key TEXT,
                    model_name TEXT NOT NULL DEFAULT 'gpt-4o-mini'
                )
            """)
        
        # Check current settings
        cursor.execute("SELECT provider_url, api_key, model_name FROM ai_settings WHERE id = 1")
        current = cursor.fetchone()
        
        if current:
            print(f"Current settings:")
            print(f"  Provider URL: {current[0]}")
            print(f"  API Key: {current[1][:20] + '...' if current[1] else 'None'}")
            print(f"  Model: {current[2]}")
        else:
            print("No existing settings found, creating new entry...")
        
        # Update or insert
        cursor.execute("""
            INSERT INTO ai_settings (id, provider_url, api_key, model_name)
            VALUES (1, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                provider_url = excluded.provider_url,
                api_key = excluded.api_key,
                model_name = excluded.model_name
        """, (OPENAI_URL, NEW_API_KEY, MODEL_NAME))
        
        conn.commit()
        
        # Verify update
        cursor.execute("SELECT provider_url, api_key, model_name FROM ai_settings WHERE id = 1")
        updated = cursor.fetchone()
        
        print("\n✓ Settings updated successfully!")
        print(f"  Provider URL: {updated[0]}")
        print(f"  API Key: {updated[1][:20] + '...' if updated[1] else 'None'}")
        print(f"  Model: {updated[2]}")
        
        return True
        
    except sqlite3.Error as e:
        print(f"Error updating database: {e}")
        conn.rollback()
        return False
    finally:
        conn.close()

def main():
    print("=" * 60)
    print("  DevChronicle API Key Update Script")
    print("=" * 60)
    print()
    
    db_path = find_database()
    
    if not db_path:
        print("❌ Database file not found!")
        print("\nPossible locations checked:")
        print(f"  - {Path.home() / '.local' / 'share' / 'com.devchronicle.app' / 'activity_logs.db'}")
        print(f"  - {Path.home() / '.local' / 'share' / 'dev-chronicles' / 'activity_logs.db'}")
        print("\nPlease run the DevChronicle app at least once to create the database.")
        return 1
    
    if update_api_key(db_path):
        print("\n✓ API key updated successfully!")
        print("\nNext steps:")
        print("  1. Restart the DevChronicle application")
        print("  2. Go to Settings and verify the API key is set")
        print("  3. Test the connection using the 'Test Connection' button")
        return 0
    else:
        print("\n❌ Failed to update API key")
        return 1

if __name__ == "__main__":
    exit(main())

