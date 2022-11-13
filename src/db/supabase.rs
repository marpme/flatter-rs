use postgrest::Postgrest;

pub fn get_supabase_client() -> Postgrest {
    Postgrest::new("https://ndblxkptxmmzkeyhrleu.supabase.co/rest/v1")
        .insert_header(
            "apikey", 
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im5kYmx4a3B0eG1temtleWhybGV1Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NjMzMTU5MTIsImV4cCI6MTk3ODg5MTkxMn0.Uy2c5RNnAnV0lSXbR3h0PcCJ5CVmqBIfmJCPSQczDZk"
        )
}
