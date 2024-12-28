DROP TABLE IF EXISTS tasks;

CREATE TABLE tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,  -- исправлено "AUTO INCREMENT" на "AUTO_INCREMENT"
    label VARCHAR(255) NOT NULL,
    description TEXT,
    is_example BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- добавлена запятая в конце строки
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP  -- добавлена запятая перед этой строкой
);
