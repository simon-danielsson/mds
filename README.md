<p align="center">
    <img src="media/logo.png" alt="mds" width="200"/>
</p>
  
<p align="center">
  <em>Markdown to static HTML slideshow generator.</em>
</p>

<p align="center">
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/mds/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#info">Info</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a>
  <br>
  <a href="#screenshots">Screenshots</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="info"></div>

## Info
  
I built this to create minimal and pretty slideshows for my Youtube channel, as
well as other contexts where a slideshow might be useful.  
  
**Features:**   
- Fancy progress-bar
- Fullscreen-mode
- Hidden scrollbar
- Auto-hide mouse after inactivity
  
**Currently supported markdown syntax:**  
- [ ] Tables
- [ ] Syntax highlighting in code blocks
- [-] Links (currently broken)
- [x] Lists (bullet, check, numbered)
- [x] Images
- [x] Code blocks
- [x] Quotes ('>')
- [x] Headers (see usage section)
  
'mds' is an abbreviation for "markdown slides".
  
---
<div id="install"></div>

## Install

``` terminal
cargo install mds-rs
```

---
<div id="usage"></div>
  
## Usage

### CLI

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
  
### Presentation controls
  
``` terminal
Left arrow      Previous slide
Right arrow     Next slide
Spacebar        Toggle fullscreen
```
  
### Markdown

No proper usage instructions written as of yet.  
See the [examples](./examples) directory for more practical information.  

``` terminal
Slide hierarchy:

    # h1            slide section
    ## h2           slide
    ### h3          slide header text
    #### h4         slide sub-header
```

---
<div id="screenshots"></div>
  
## Screenshots
   
No screenshots added yet.
    
---
<div id="license"></div>
  
## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/mds/blob/main/LICENSE).  
