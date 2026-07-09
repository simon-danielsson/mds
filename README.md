
### info

mds (markdown slides)



### todo

- add support for tables


``` terminal

Slide hierarchy:

    # h1            slide section
    ## h2           slide
    ### h3          slide header text
    #### h4         slide sub-header

```


``` terminal
USAGE
    mds [options] <src-file>

META OPTIONS
    -h, --help        Displays this help message.
    -v, --version     Display current version and other information.

OPTIONS
    -o <path>         Output destination path surrounded by double quotes.
                      Example: mdp -o "../my-presentation.html" present.md
                      If option is omitted, the slide is generated to 
                      the current directory. 
```
