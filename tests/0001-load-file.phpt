--TEST--
include external file
--FILE--
<?php
include __DIR__ . '/integration/0001-load-file.php';
?>
--EXPECT--
bool(false)
int(1)
