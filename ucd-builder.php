<?php

$ucd_path = 'ucd.nounihan.flat.xml';
$ucd_chars = [];

if (file_exists($ucd_path)) {
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
    echo "https://www.unicode.org/Public/14.0.0/ucdxml/ucd.nounihan.flat.zip";
    exit();
}

?>
pub struct UCEntry {
    pub cp: u32,
    pub c: char,
    pub na: &'static str,
    pub na1: &'static str,
}

pub fn get_uc_table() -> [UCEntry; <?= count($ucd_chars) ?>] {
    const UC_TABLE: [UCEntry; <?= count($ucd_chars) ?>] = [
<?php foreach ($ucd_chars as $i => $char): ?>
        UCEntry { cp: 0x<?= dechex(hexdec($char->cp)) ?>, c: '\u{<?= dechex(max(hexdec($char->cp), 0x20)) ?>}', na: "<?= $char->na ?>", na1: "<?= $char->na1 ?>" },
<?php endforeach; ?>
    ];
    return UC_TABLE;
}
