SELECT comments.id,
      user_id,
      content,
      username
FROM   comments
      JOIN users
        ON user_id = users.id
WHERE  deleted = false
      AND bark_id = ?;  
