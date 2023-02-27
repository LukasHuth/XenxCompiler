# Xenx Compiler

## AST structure

```
{
    name: function decleration
    childs: 
    [
        {
            "name": int
            childs: []
        }
        {
            "name": body
            childs:
            [
                {
                    name: decleration
                    childs:
                    [
                        {
                            name: int
                            childs: []
                        }
                        {
                            name: a
                            childs: []
                        }
                        {
                            name: 2
                            childs: []
                        }
                    ]
                }
                {
                    name: return
                    childs:
                    [
                        {
                            name: identifier
                            childs:
                            [
                                {
                                    name: a
                                    childs: []
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    ]
}
```