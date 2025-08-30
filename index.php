<?php
require_once('common.php');

$lines = file(ROOT. "/db.csv");
$db = [];

foreach ($lines as $l) {
  $fields = explode(";", $l);
  if (!isset($db[$fields[1]]) || $db[$fields[1]][3] != "master")
    $db[$fields[1]] = $fields;
}

function parseHostgroup($f) {
  $lines = file(ROOT. "/$f");
  $hosts = [];
  for ($i = 0; $i < count($lines); $i++) {
    if (substr($lines[$i],0,2) == "**") {
      $hosts[trim(substr($lines[$i], 2))] = trim($lines[$i+1]);
    }
  }

  return $hosts;
}

function getHostsWithLLDPInfo() {

  $path = FACTS. "/hosts";
  $lldpPath = FACTS. "/lldp";
  $swPath = FACTS. "/switches";

  $files = scandir($swPath);
  $switches = [];
  foreach ($files as $f) {
    if (substr($f, -5) != ".json") continue;

    $json = @json_decode(@file_get_contents("$swPath/$f"));
    if (isset($json->name) && isset($json->mac)) {
      if (!is_array($json->mac)) {
        $json->mac = [$json->mac];
      }

      foreach ($json->mac as $ma) {
        $switches[$ma] = $json;
      }
    }
  }

  $files = scandir($path);

  $hosts = [];
  foreach ($files as $f) {
    if (substr($f, -5) != ".json") continue;

    $json = @json_decode(@file_get_contents("$path/$f"));
    if (strlen(@$json->hostName) < 1) continue;
    $hostName = $json->hostName;

    $uuid = @$json->uuid;
    $isVMware = stripos(@$json->vendor, "vmware") !== false;
    $location = null;
    $mac = null;
    $port = null;
    if (!$isVMware && is_readable("$lldpPath/$uuid.json")) {
      $json = @json_decode(@file_get_contents("$lldpPath/$uuid.json"));
      $chassis = @$json->lldp[0]->interface[0]->chassis[0];
      $port = @$json->lldp[0]->interface[0]->port[0];
      if (is_array($chassis->id)) {
        foreach ($chassis->id as $id) {
          if ($id->type == "mac") {
            $mac = $id->value;
          }
        }
      }
      if (is_array($port->id)) {
        foreach ($port->id as $id) {
          if ($id->type == "ifname") {
            $port = $id->value;
          }
        }
      }

      if (isset($switches[$mac])) {
        $s = $switches[$mac];
        // location can be determined just by knowing the switch
        if (isset($s->location)) {
          $location = $s->location;
        }
        // the switch alone is not enough. we have to look at the ports as well
        else if (isset($port) && isset($s->{"port-match"})) {
          $matches = get_object_vars($s->{"port-match"});
          foreach ($matches as $key => $val) {
            if (stripos($port, $key) !== false && isset($val->location)) {
              $location = $val->location;
            }
          }
        }
      }
    }

    if ($isVMware) {
      $hosts[$hostName] = "VM";
    } else {
      $hosts[$hostName] = isset($location) ? "M". $location->datacenter. " K". $location->kube : "?";
    }
  }

  return $hosts;
}


$lldpHosts = getHostsWithLLDPInfo();
$acolors = ['#C0C0C0', '#FFFF00', '#FFCC00', '#FF9900', '#CCFF00', '#CCCC00', '#CC99FF', '#FF00FF', '#FF0000', '#33FFFF', '#CCFFFF'];
$mcolors = ['unknown'=>'#ffffff'];

echo "<p>";
echo "<b>Note:</b> The <b>rev</b> column displays the <em>earliest</em> known (from CI) revision that produces a given <b>system</b> path. That means that: Given two revs; A and B, which both produce the same system path, and of which A is the earliest known (that is, built by CI), an \"update\" from A to B will <em>not</em> result in the rev column being updated.";
echo "</p>";

echo "<table cellpadding=\"5\">";

$cGroups = 0;
$cHosts = 0;

foreach (scandir(ROOT) as $f) {
  if (!is_file(ROOT. "/$f") || $f == "db.csv") continue;
  $hosts = parseHostgroup($f);
  if (count($hosts) < 1) continue; // we don't count (nor show) _empty_ host groups

  $cGroups++;

  echo "<thead style=\"font-size: 16pt;\"><th colspan=\"4\"><hr>$f</th></thead>";
  echo "<thead style=\"text-align: left; font-size: 14pt;\"><th>host</th><th>loc</th><th>system</th><th>rev</th><th>ref</th></thead>";

  echo "<tbody>";

  foreach ($hosts as $h=>$sys) {
    $cHosts++;

    echo "<tr>";
    $rev = isset($db[$sys]) ? $db[$sys][2] : "unknown";
    $git = isset($db[$sys]) ? '<a style="color: #000000;" href="https://gitlab.dbc.dk/platform/deployments/commit/'. $db[$sys][2]. '">'. $db[$sys][2]. '</a>' : 'unknown';
    $ref = isset($db[$sys]) ? $db[$sys][3] : "";
    $color = isset($mcolors[$rev]) ? $mcolors[$rev] : array_pop($acolors);
    $color = isset($color) ? $color : "#ffffff";
    $mcolors[$rev] = $color;

    $ref = ($ref != "" && $ref != "master") ? '<a href="https://gitlab.dbc.dk/platform/deployments/compare/master...'. $ref. '">'. $ref. '</a>' : $ref;
    echo "<td><a href=\"history.php?host=$h\">". $h. "</a></td><td>". $lldpHosts[$h]. "</td><td>". minimizeSysName($sys). "</td><td style=\"background-color: $color; color: #000000;\">". $git. "</td><td>". $ref. "</td>";
    echo "</tr>";
  }

  echo "</tbody>";

}
echo "</table>";

echo "<hr>";

echo "<h3>Total hosts: $cHosts</h3>";
echo "<h3>Total hostgroups: $cGroups</h3>";
