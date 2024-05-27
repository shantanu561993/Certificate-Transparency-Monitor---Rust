# Certificate Transparency Monitor

This project is an attempt to monitor certificate transparency logs with purpose of bug bounty or bug hunting. The idea is to monitor the logs with bunch of domain wildcards which are generally found in the Bug Bounty Platforms

## Installation

You can download the release or build your own. To build run 

```bash
cargo build --release 
```

## Usage
### Help
```cmd
C:\Downloads>ctlrust.exe --help
Cyber Security Researcher

Usage: ctlrust.exe [OPTIONS]

Options:
      --regex <REGEX>  A regex pattern to monitor. Example .*\.com for monitoring all .com domains
      --url <URL>      A URL to list of wildcards of bugbounty.
  -h, --help           Print help
  -V, --version        Print version
```
Running to monitor all domains
```cmd
C:\Downloads>ctlrust.exe

{"leaf": ["goodday-studio.co.uk"], "all_domains": ["goodday-studio.co.uk"]}
{"all_domains": ["lizottebookkeeping.com"], "leaf": ["lizottebookkeeping.com"]}
{"all_domains": ["mdawaghreh.com"], "leaf": ["mdawaghreh.com"]}
{"leaf": ["www.vingtpieds.com"], "all_domains": ["vingtpieds.com", "www.vingtpieds.com"]}
{"leaf": ["checkout.modefa.shop"], "all_domains": ["checkout.modefa.shop"]}
{"leaf": [], "all_domains": ["ugwcscane003f19fb23abd88886417e24d8a9afc.wwwwww.mx3.node1.dxcbyd.lon1.couchdrop.io"]}
{"leaf": ["mwww.wwwwww.www.www.elasticsearch.uat.colburn.dev"], "all_domains": ["mwww.wwwwww.www.www.elasticsearch.uat.colburn.dev"]}
{"leaf": ["moodle.tswmps.edu.hk"], "all_domains": ["moodle.tswmps.edu.hk"]}
{"all_domains": ["www.www.blog.blog.blog.blog.bicicletasilgiro.bluecaribu.chat"], "leaf": ["www.www.blog.blog.blog.blog.bicicletasilgiro.bluecaribu.chat"]}
{"leaf": ["www.kuahdalca.standwith.info"], "all_domains": ["www.kuahdalca.standwith.info"]}
```

Running to monitor Bug Bounty Wildcard Targets
```
https://github.com/arkadiyt/bounty-targets-data/blob/main/data/wildcards.txt
```
```cmd
C:\Downloads>ctlrust.exe --url https://raw.githubusercontent.com/arkadiyt/bounty-targets-data/main/data/wildcards.txt

*.1.oca.nflxvideo.net
*.1.live.nflxvideo.net
*.1.nflxso.net
*.1.oca.nflxvideo.net
*.1.live.nflxvideo.net
*.1.nflxso.net
*.1.nflxso.net
*.1.nflxso.net
*.1.nflxso.net
*.1.nflxso.net
*.1.oca.nflxvideo.net
*.1.live.nflxvideo.net
*.1.nflxso.net
*.1.oca.nflxvideo.net
*.1.live.nflxvideo.net
*.1.nflxso.net
*.1.nflxso.net
*.1.nflxso.net
*.1.nflxso.net
ipv6-c088-ord001-dev-ix.1.oca.nflxvideo.net
ipv6-only-c088-ord001-dev-ix.1.oca.nflxvideo.net
ipv4-c088-ord001-dev-ix.1.oca.nflxvideo.net
WKSP000CDDC9.europe.ups.com
WKSP000CDDC9.europe.ups.com
```

Running to monitor a custom regex
```cmd
C:\Downloads>ctlrust.exe --regex .*\.com

productphotos4you.com
www.productphotos4you.com
wwwwww.skydrive.novago.com.br
wfgdsubis.213.com
www.jenkins.mwwwwwwwww.wwwsecure.org.domains.app.bigbeartechworld.com
autodiscover.theorganizedxperience.com
cpanel.theorganizedxperience.com
cpcalendars.theorganizedxperience.com
cpcontacts.theorganizedxperience.com
```
## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
