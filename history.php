<?php
require_once('common.php');

if (!isset($_GET['host']))
	die("GET-param 'hostname' not set");

$hostname = basename($_GET['host']);
$filename = ROOT. "/history/$hostname";

if (!is_readable($filename))
	die("no history data for host: $hostname");

echo "<h1>$hostname</h1>";
$lines = file($filename); 

$entries = [];
foreach ($lines as $l) {
	$fields = explode(';',trim($l));
	$timestamp = strtotime(@$fields[0]);
	if ($timestamp < 1 || count($fields) < 4) {
		echo "skipping timed entry that doesn't make sense: $l";
		continue;
	}
	
	$entries[$timestamp] = [
		"user" => $fields[1],
		"system" => $fields[2],
		"action" => $fields[3],
		"time" => $timestamp,
		"gitrev" => "" // TODO
	];
}

krsort($entries, SORT_NUMERIC);
$entries = array_values($entries);

echo "<table cellpadding=\"5\">";

$date = null;
for ($i = 0; $i < count($entries); $i++) {
	$e = $entries[$i];
	$next = @$entries[$i+1];
	$sysname = minimizeSysName($e["system"]);
	$syslink = is_array($next) && $e["system"] != $next["system"] ? '<a href="nix-diff.php?pathfrom='. $next["system"]. '&pathto='. $e["system"]. '">'. $sysname. '</a>' : $sysname;
	$time = $e['time'];
	$newDate = date("l, Y-m-d", $time);
	if ($date != $newDate) {
		if (isset($date)) {
			echo "</tbody>";
		}
		$date = $newDate;
		echo "<thead style=\"font-size: 16pt;\"><th colspan=\"5\"><hr>$date</th></thead>";
		echo "<thead style=\"text-align: left; font-size: 14pt;\"><th>time</th><th>user</th><th>system</th><th>action</th><th>git-ref</th></thead>";
		echo "<tbody>";
	}
	echo "<tr>";
	echo "<td>". date("H:i:s T", $time). "</td><td>". $e["user"]. "</td><td>". $syslink. "</td><td>". $e["action"]. "</td><td>". $e['gitrev']. "</td>";
	echo "</tr>";
}

echo "</tbody>";
echo "</table>";

