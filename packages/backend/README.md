# Rapinsel (backend)

## Requests with cURL

`/GET`

```sh
curl --location 'http://127.0.0.1:5174/api/v1/health-check'
curl --location 'http://127.0.0.1:5174/api/v1/faqs'
curl --location 'http://127.0.0.1:5174/api/v1/faqs/{uuid}'
curl --location 'http://127.0.0.1:5174/api/v1/users'
```

`/POST`

```sh
curl --location 'http://127.0.0.1:5174/api/v1/faqs' \
--header 'Content-Type: application/json' \
--data '{
    "question": "Is this app free open-source?",
    "answer": "Yes, open-source and forever free."
}'
```

`/PUT`

```sh
curl --location --request PUT 'http://127.0.0.1:5174/api/v1/faqs/{uuid}' \
--header 'Content-Type: application/json' \
--data '{
    "question": "Is this app free and open-source?",
    "answer": "Ofcourse, open-source and forever free."
}'
```
