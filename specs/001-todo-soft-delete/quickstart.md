# Quickstart: Soft Delete and Audit Metadata for To-Do Items

## 1. Configure audit access

Add an audit token to local configuration or environment variables.

Example environment variable:

```powershell
$env:MICROSERVICE__AUDIT__TOKEN="local-audit-token"
```

## 2. Run the service

```powershell
cargo run -p starter
```

## 3. Create a to-do item

```bash
curl -X POST http://localhost:8181/api/v1/to-do-items \
  -H "Content-Type: application/json" \
  -d "{\"title\":\"Buy milk\",\"note\":\"2 liters\",\"status\":\"pending\"}"
```

## 4. Soft-delete the item

```bash
curl -X DELETE http://localhost:8181/api/v1/to-do-items/{id}
```

## 5. Verify standard access hides the item

```bash
curl http://localhost:8181/api/v1/to-do-items/{id}
```

Expected result: `404 Not Found`

## 6. Retrieve the deleted item through the audit path

```bash
curl http://localhost:8181/api/v1/audit/to-do-items/{id} \
  -H "X-Audit-Token: local-audit-token"
```

Expected result: `200 OK` with deletion metadata included.
