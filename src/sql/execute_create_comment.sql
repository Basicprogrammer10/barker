INSERT INTO comments
           (id,
            bark_id,
            user_id,
            content,
            deleted,
            date)
VALUES      (?,
            ?,
            ?,
            ?,
            false,
            Strftime('%s', 'now'));
