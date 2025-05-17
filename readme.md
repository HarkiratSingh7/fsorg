# fsorg - File organizer
This utility can be used to organise files into directories using regex pattern matching for unorganised files.

### Example
``` bash
fsorg -s ../backup -d ~/ -c ../fsorg.json 
```
or if there is no configurations then use default configurations by omitting -c, it will also generate a default fsorg.json
``` bash
fsorg -s ../backup -d ~/  
```

### Syntax
```bash
fsorg [OPTIONS]
Organizes the files according to user's rules based on regex file name matching.

--load-config-file | -c Loads the config file having the rules for file organization.
                        If not provided then it generates a basic fsorg.json which user can modify.

--source-dir | -s       Source directory containing unorganised files. (By default it is current working directory).

--destination-dir | -d  Destination where to place the organized files. (By default it is current working directory).

```

### Sample config
```json
{
  "rules": {
    "(?i)^.*\\.(jpg|jpeg|png|gif|bmp|webp|tiff?)$": "Images",
    "(?i)^.*\\.(pdf|docx?|xlsx?|pptx?|odt|ods|txt|rtf|csv|md)$": "Documents"
  },
  "version": "0.0.1"
}
```