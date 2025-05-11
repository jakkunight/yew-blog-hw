{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env = {};

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.cargo-binstall
    pkgs.cargo-binutils
    pkgs.trunk
    pkgs.tailwindcss
  ];

  # https://devenv.sh/scripts/
  scripts = {
    deploy.exec = ''
      echo "[INFO] Building application..."
      trunk build --release --public-url /yew-blog-hw/
      echo "[INFO] Building CSS assets..."
      bun run build
    '';
  };

  enterShell = ''
  '';

  # https://devenv.sh/tests/
  enterTest = ''
  '';

  # https://devenv.sh/services/
  services = {
    #    postgres = {
    #      enable = true;
    #      initialDatabases = [
    # {
    #   user = "user";
    #   pass = "password";
    #   name = "columbia-moodle";
    #   schema = ./columbia-moodle.sql;
    # }
    #      ];
    #    };
  };

  # https://devenv.sh/languages/
  languages = {
    nix = {
      enable = true;
    };
    rust = {
      enable = true;
      channel = "stable";
      targets = [
        "wasm32-unknown-unknown"
        "x86_64-unknown-linux-gnu"
      ];
    };
    typescript = {
      enable = true;
    };
    javascript = {
      enable = true;
      bun = {
        enable = true;
      };
      npm = {
        enable = true;
      };
    };
  };

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
