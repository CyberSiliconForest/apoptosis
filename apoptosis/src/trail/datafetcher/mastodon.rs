
const USER_FETCH_QUERY: &str = """
SELECT
  accounts.id AS id,
  accounts.username AS username,
  accounts.private_key AS private_key
FROM accounts
INNER JOIN users ON accounts.id = users.account_id
WHERE users.disabled IS NOT TRUE
""";

//