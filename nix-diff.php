<?php
require_once('common.php');
require_once __DIR__.'/vendor/autoload.php';

use SensioLabs\AnsiConverter\AnsiToHtmlConverter;

function verify_path($path) {
	if (!preg_match("/^\/nix\/store\/[a-z0-9.-]+$/", $path))
		throw new InvalidArgumentException("invalid system path: $path");

	if (!is_dir($path))
		throw new InvalidArgumentException("system path: $path does not exist or is not a directory");

	return $path;
}

function verify_drv($path) {
	if (!preg_match("/^\/nix\/store\/[a-z0-9.-]+\.drv$/", $path))
		throw new InvalidArgumentException("invalid system derivation: $path");

	if (!is_file($path))
		throw new InvalidArgumentException("system derivation: $path does not exist or is not a regular file");

	return $path;
}

function deriver($path) {
	$path = escapeshellarg($path);
	$out = [];
	$code = -1;
	
	exec('nix-store --query --deriver '. $path, $out, $code);
	if ($code == 0 && count($out) > 0) {
		return end($out);
	} else {
		throw new RuntimeException("deriver, exit code: $code, output: ". implode("\n", $out));
	}
}

function nix_diff($from, $to) {
	$from = escapeshellarg($from);
	$to = escapeshellarg($to);
	$out = [];
	$code = -1;
	
	exec('nix-diff --color always '. $from. ' '. $to, $out, $code);
	if ($code == 0) {
		return $out;
	} else {
		throw new RuntimeException("nix diff, exit code: $code, output: ". implode("\n", $out));
	}
}


if (isset($_GET['pathfrom']) && isset($_GET['pathto'])) {

	$drv_from = verify_drv(deriver(verify_path($_GET['pathfrom'])));
	$drv_to = verify_drv(deriver(verify_path($_GET['pathto'])));

} else if (isset($_GET['drvfrom']) && isset($_GET['drvto'])) {

	$drv_from = verify_drv($_GET['drvfrom']);
	$drv_to = verify_drv($_GET['drvto']);

} else {
	die("missing args 'pathfrom', 'pathto' or 'drvfrom', 'drvto'"); 
}


$output = nix_diff($drv_from, $drv_to);

$converter = new AnsiToHtmlConverter();
?>

<html>
<head>

<style type="text/css">
.console {
	background-color: #000000;
	margin: 20px;
	line-height: 20pt;
	font-family: monospace;
	font-size: 10pt;
	color: #ffffff;
 }
pre {
	padding: 10px;
	color: #ff0000;
	background-color: #333333;
}
</style>

</head>

<body class="console">
<div>
	<?php // echo nl2br($converter->convert(implode("\n", $output))); ?>
	<p>Currently we cannot show you the system diff directly, but if you are privileged enough, you can try this snippet:</p>
	<pre>ssh nix-build-p03 "nix-shell -p nix-diff --run 'nix-diff --color=always <?php echo "$drv_from $drv_to"; ?>'"</pre>
</div>
</body>
