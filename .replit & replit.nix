# .replit
modules = ["nodejs-20", "python-3.11", "rust"]
run = "npm run start:hybrid"

[nix]
channel = "stable-23_11"

[deployment]
run = ["npm", "run", "start:hybrid"]


# replit.nix
{ pkgs }: {
    deps = [
        pkgs.nodejs_20
        pkgs.python311
        pkgs.python311Packages.pip
        pkgs.rustc
        pkgs.cargo
        pkgs.cargo-audit
    ];
}
