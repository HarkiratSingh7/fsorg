# fsorg - File organiser
This utility can be used to organise files into directories using regex pattern matching for unorganised files.

### Example
#### Organising
``` bash
fsorg -s ../backup -d ~/ -c ../fsorg.json 
```
or if there is no configurations then use default configurations by omitting -c, it will also generate a default ~/.fsorg.json in user's home directory
``` bash
fsorg -s ../backup -d ~/  
```
#### Viewing the rules
```bash 
fsorg -v
```
<pre>
+-------------------------------------------------------- | -------------------+
|Regex                                                    |        Destinations|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(mp4|mkv|flv|avi|mov)$                          |              Videos|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(exe|msi|deb|rpm|sh|bat)$                       |          Installers|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(mp3|wav|ogg|flac)$                             |               Music|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(zip|rar|7z|tar\.gz|tar\.bz2)$                  |            Archives|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(jpg|jpeg|png|gif|bmp|webp|tiff?)$              |              Images|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(pdf|docx?|xlsx?|pptx?|odt|ods|txt|rtf|csv|md)$ | Documents and Files|
+-------------------------------------------------------- | -------------------+
|(?i)^.*\.(rs|cpp|c|h|hpp|py|java|go|rb|cs|swift)$        |                Code|
+-------------------------------------------------------- | -------------------+
</pre>

#### Adding a rule 
```bash
fsorg -a "(?i)^.*\.(mp3|wav|ogg|flac)$", "Videos"
```

#### Deleting a rule
```bash
fsorg -r "(?i)^.*\.(mp3|wav|ogg|flac)$"
```
### Syntax
```
Authors: Harkirat Singh (honey.harkirat@outlook.com)
Version: 1.0.0
Syntax: fsorg [OPTIONS]
                 --config | -c Specify the json config file. By default the config is present at ~/.fsorg.json
             --source-dir | -s Source directory containing unorganised files. (By default it is current working directory).
        --destination-dir | -d Destination where to place the organized files. (By default it is current working directory).
               --add-rule | -a Adds a rule: fsorg -a "(?i)^.*\.(mp3|wav|ogg|flac)$", "Videos"
            --remove-rule | -r Removes a rule: fsorg -r "(?i)^.*\.(mp3|wav|ogg|flac)$"
             --view-rules | -v Views the current rules present in specified or default configs.
                --dry-run | -p Creates an action plan for organising the files: fsorg [OTHER OPTIONS] -p plan1.txt
                --execute | -x Executes the provided plan: fsorg -x plan1.txt
```

### Sample config
```json
{
  "rules": {
    "(?i)^.*\\.(rs|cpp|c|h|hpp|py|java|go|rb|cs|swift)$": "Code",
    "(?i)^.*\\.(exe|msi|deb|rpm|sh|bat)$": "Installers",
    "(?i)^.*\\.(pdf|docx?|xlsx?|pptx?|odt|ods|txt|rtf|csv|md)$": "Documents and Files",
    "(?i)^.*\\.(jpg|jpeg|png|gif|bmp|webp|tiff?)$": "Images",
    "(?i)^.*\\.(zip|rar|7z|tar\\.gz|tar\\.bz2)$": "Archives",
    "(?i)^.*\\.(mp3|wav|ogg|flac)$": "Music",
    "(?i)^.*\\.(mp4|mkv|flv|avi|mov)$": "Videos"
  },
  "version": "0.1.0"
}
```