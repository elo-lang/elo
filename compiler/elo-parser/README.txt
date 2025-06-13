Elo Parser Implementation
Copyright (c) 2025 Igor Ferreira, Marcio Dantas

       Operator precedence
+-------+-------------------------+
| Level | Operations              |
+-------+-------------------------+
| 9     | unary &, !, ~, Unary -  |
| 8     | <<, >>                  |
| 7     | *, /, %                 |
| 6     | +, -                    |
| 5     | ^, |, &                 |
| 4     | &&, ||                  |
| 3     | <, >, <=, >=            |
| 2     | ==, !=                  |
| 1     | =                       |
+-------+-------------------------+

The higher the level is, the higher is the precedence 