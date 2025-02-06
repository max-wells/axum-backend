

```sql
psql -U postgres

-- 1. First create the user
CREATE USER my_user WITH PASSWORD 'password';

-- 2. Create the database
CREATE DATABASE axum_backend_todos;

-- 3. Grant privileges to my_user
GRANT ALL PRIVILEGES ON DATABASE axum_backend_todos TO my_user;

-- 4. Connect to the database (make sure to type this exactly)
\c axum_backend_todos

-- 5. After connecting to axum_backend_todos, run these:
GRANT ALL ON SCHEMA public TO my_user;
GRANT ALL ON ALL TABLES IN SCHEMA public TO my_user;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO my_user;

-- 6. Set default privileges
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO my_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO my_user;

```







```bash
# -----------------------------------------------------------------------------
# Database (PostgreSQL)
# -----------------------------------------------------------------------------
DATABASE_URL=postgresql://my_user:password@localhost:5432/axum_backend_todos
POSTGRES_PASSWORD=password
```
 



