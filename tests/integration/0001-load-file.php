<?php
$doc = LoPdf\Document::open(__DIR__ . '/../data/example.pdf');

var_dump($doc->isEncrypted());
var_dump($doc->countPages());
