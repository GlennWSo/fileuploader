
{
  inputs =  {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
  };

  outputs = { self, nixpkgs, rust-overlay, systems } @ inputs:
  let 
    forEachSystem = nixpkgs.lib.genAttrs (import systems);

  in
  {
    devShells = forEachSystem(system:
      let 
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.nightly.latest.default;
        tools = with pkgs; [
          vscode-langservers-extracted
          rust
          bacon
          rust-analyzer
          cargo-watch
        ];
      in
        { 
        default = pkgs.mkShell {
          name = "axum";
          buildInputs = tools;
          
          
        };
      }
    );
  };
}



        
