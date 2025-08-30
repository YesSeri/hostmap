<?php
date_default_timezone_set('Europe/Copenhagen');

define("BARE", "/var/lib/gitlab-runner");
$reposPath = BARE. "/hostoverview-repos";

$systemsPath = isset($_ENV['HO_SYSTEMS_PATH']) ? $_ENV['HO_SYSTEMS_PATH'] : BARE. "/systems";
$deploymentsPath = isset($_ENV['HO_DEPLOYMENTS_PATH']) ? $_ENV['HO_DEPLOYMENTS_PATH'] : "$reposPath/deployments";
$factsPath = isset($_ENV['HO_FACTS_PATH']) ? $_ENV['HO_FACTS_PATH'] : "$deploymentsPath/facts";

define("ROOT", $systemsPath);
define("DEPLOYMENTS", $deploymentsPath);
define("FACTS", $factsPath);

function minimizeSysName($sys) {
  // /nix/store/qf2k7wgdassswwacaixwmgn75m03ngyv-nixos-system-hydra-p01-19.09pre-git
  return preg_replace("/^\/nix\/store\/([a-z0-9]{10})[a-z0-9]+-nixos-system-([0-9a-z-._]+)(pre|post)-git$/","$1-$2",$sys);
}

