# Execute view function of a module

**`POST /view`**

Call functions defined as view.

## Request Body

| Name | Type | Required | Description |
|------|------|----------|-------------|
| function | string | ✓ | Field for entering the view function defined in the deployed module. Enter in the following format: - Format: {address}::{module_name}::{function_name} |
| type_arguments | array | ✓ | Field for entering the function's type arguments. For functions that do not take generic type arguments, enter an empty array. |
| arguments | array | ✓ | Field for entering the function's arguments. For functions that take no arguments, enter an empty array in this field. |

## Response

### Example

```json
[
  4000000000
]
```
