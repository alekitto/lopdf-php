<?php
$doc = LoPdf\Document::open(__DIR__ . '/../data/example.pdf');

var_dump($doc->compress());
var_dump($doc->save());
