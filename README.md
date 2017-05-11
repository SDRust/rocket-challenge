***setup**

* rustup override set nightly


***Challanges**

* Change the CSV parsing to load the headers and slugify then return json based on the headers (allowing any json file to be returned)
* Identify image URLs in the CSV a new endpoint to serve the images. Then convert the URL returned in the JSON to the URL for the image served via rocket
* Add an endpoint to allow full text search via https://github.com/tantivy-search/tantivy ( you will probably need an additional endpoint to create the index)