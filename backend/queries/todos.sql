-- name: ListTodos :many
SELECT id, title, completed, created_at FROM todos ORDER BY created_at DESC;
-- name: CreateTodo :one
INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed, created_at;
-- name: UpdateTodo :exec
UPDATE todos SET completed = $2 WHERE id = $1;
-- name: DeleteTodo :exec
DELETE FROM todos WHERE id = $1;
