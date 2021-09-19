let books_by_popularity = "
WITH orders as (SELECT book_id, COUNT(*) as c FROM links_requests GROUP BY book_id)
SELECT bl.* FROM book_links as bl
LEFT JOIN orders as o ON o.book_id = bl.id
ORDER BY o.c DESC, bl.id ASC
"
