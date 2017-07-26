# Rust Campaigns Server

This is intended for teams in the Rust Project to setup calls to action and outreach advertising campaigns to be displayed in the Rust Web network. This application takes some inspiration from the [Perl Community Ad Server](http://pcas.szabgab.com/)

The campaigns will not be commercial. We don't accept money for displaying these campaigns across the Web sites.

The content of the ads could be:

- Calls to action and/or participation from different projects in the ecosystem (probably automatically imported from TWiR)
- Events
- Surveys

Initially, we just want to serve simple campaigns. There will not be any tracking, targetting, scheduling, frequency management, custom sizing, categorization or reporting initially. All of this could be implemented in the future.

## Usage

### Adding campaigns

Please create an issue in this repository or contact `community-team@rust-lang.org` with a title, start and end date, and a destination URL for your campaign. You can also add a brief description to it.

### Displaying Rust Campaigns in your website

There are several options to display Rust campaigns in a website.

#### Using the provided script

You can use the `campaigns.js` file located at the root of the _Rust Campaigns Server_ (TBD) and put it in the place where you want the campaigns to be rendered. It will render a randomized set of active campaigns.

The script response will be cached for 6 hours by default. You can use a random cache bust parameter to override this. *This is not yet implemented*.

The script can also get an `l=n` parameter to indicate the number of campaigns to load. For example, `https://<rcs-server-url>/campaigns.js?l=10` will load at most 10 randomized active campaigns.

##### Styling campaigns with the provided script

The provided script renders a `div` component for each campaign. This is the markup for such element:

```
<div class="rcs-campaign">
    <h3><a href="CLICK-URL">TITLE</a><h3>
    <div class="rcs-description">
        DESCRIPTION
    </div>
</div>
```

#### Using the API

There is a simple API available at `/api/v1/` you can use to fetch campaigns if you want to implement custom rendering. The `/api/v1/campaigns` endpoint will behave just as the provided script and will return a JSON array of randomized active campaigns.

You can also use the `l` parameter to indicate the number of campaigns to load.

#### Using the reference JavaScript client

*Note: not implemented*

The reference JavaScript client implements communication with the API so you don't have to. Then, you can use it to easily fetch the data and implement custom rendering of the campaigns.

```
var rcsClient = new RCSClient();
var rustCampaigns = rcsClient.load(5);
...
```

### Deploy your own instance on Heroku

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)
