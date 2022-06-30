#!/usr/bin/env php
<?php

$ucd_path = isset($argv[1]) ? $argv[1] : false;
$ucd_chars = [];

if ($ucd_path && file_exists($ucd_path)) {
    $doc = simplexml_load_file($ucd_path);
    $doc->registerXPathNamespace('ns', 'http://www.unicode.org/ns/2003/ucd/1.0');
    $ucd_chars = array_map(function($char) {
        return (object)$char->attributes();
    }, $doc->xpath('//ns:char'));
    $ucd_chars = array_filter($ucd_chars, function($char) {
        return $char->cp && (strlen($char->na) > 0 || strlen($char->na1) > 0);
    });
    $ucd_chars = array_values($ucd_chars);
}
else {
    echo "You must first download the ucd xml file from:\n";
    echo "https://www.unicode.org/Public/14.0.0/ucdxml/ucd.nounihan.flat.zip\n\n";
    echo "You can see other versions at:\n";
    echo "https://www.unicode.org/Public/\n\n";
    echo "Pass this file as the first argument to the ucd-builder.php script.";
    exit();
}

?>
pub struct UCEntry {
    pub cp: u32,
    pub c: char,
    pub na: &'static str,
    pub na1: &'static str,
}

pub fn get_uc_table(start: u32, end: u32) -> &'static [UCEntry] {
    const UC_TABLE: [UCEntry; <?= count($ucd_chars) ?>] = [
<?php foreach ($ucd_chars as $i => $char): ?>
        UCEntry { cp: 0x<?= dechex(hexdec($char->cp)) ?>, c: '\u{<?= dechex(max(hexdec($char->cp), 0x20)) ?>}', na: "<?= $char->na ?>", na1: "<?= $char->na1 ?>" },
<?php endforeach; ?>
    ];

    if start == 0 && end == 0 {
        return &UC_TABLE;
    }
    else {
        let mut ustart: usize = 0;
        let mut uend: usize = 0;

        for (i, c) in UC_TABLE.iter().enumerate() {
            if c.cp >= start {
                ustart = i;
                break;
            }
        }
        for (i, c) in UC_TABLE.iter().enumerate() {
            if c.cp == end {
                uend = i + 1;
                break;
            }
            else if c.cp > end {
                uend = i;
                break;
            } 
        }
        return &UC_TABLE[ustart..uend];
    }
}
