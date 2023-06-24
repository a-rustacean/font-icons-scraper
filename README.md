# Font Icons Scraper

Companies like Font Awesome leverage the concept of icon fonts to provide a convenient CDN solution for their font assets. Using
scalable vector icons as font glyphs is made possible by icon fonts, making it simple to integrate and style using CSS. However,
it is not feasible to access the entire collection of SVG icons from a premium CDN without the necessary authorisation.

With the help of this tool, you can extract SVG icons directly from the font files used by services like Font Awesome. You can
utilise the SVG icons independently, modify their properties, and incorporate them into your applications without depending on a
particular CDN or font supplier if you extract the SVG icons. This gives you the ability to adapt and maximise the use of symbols
to comply with

## Features

- Extract SVG icons from font files
- Gain direct access to individual SVG icons
- Customize and optimize icon usage
- Enhance the visual appeal of your projects

## Installation

To install the font icons scraper, follow, these steps:

1. Make sure your have rust installed on your machine.
2. Clone this repository to your local machine.
3. Navigate to the project directory.
4. Run the provided installation script:

```shell
./install.sh
```

This will set up the neccessary dependencies and configurations for the scraper.

## Usage

Once the installation is complete, you can run the html scraper with the following command.

```shell
scrap-icons {CSS URL} {OUTPUT FOLDER}
```

Here are some exaple URLs you can use:
- https://site-assets.fontawesome.com/releases/v6.4.0/css/all.css
- https://site-assets.fontawesome.com/releases/v6.4.0/css/sharp-solid.css
- https://site-assets.fontawesome.com/releases/v6.4.0/css/sharp-regular.css
- https://site-assets.fontawesome.com/releases/v6.4.0/css/sharp-light.css

The tool will start extracting SVG icons from the font files, providing you with individual SVG files for each icon.
You can customize the extraction behavior or specify font files as needed within the code.

## Uninstall

To uninstall the icons scraper, follow these steps:

1. Navigate to the project directory.
2. Run the provided uninstallation script:

```shell
./uninstall.sh
```

## Contributing

Contributions to this project are welcome! If you find any issues, have suggestions for improvements, or would like to add missing features,
please feel free to submit a pull request. Although there is no strict PR template, please provide a clear description and follow best
practices for code contributions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
