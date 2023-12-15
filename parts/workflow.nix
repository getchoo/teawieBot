{
  githubWorkflowGenerator = {
    outputs = [
      "checks"
      "devShells"
      "packages"
    ];

    overrides = {
      checks.systems = ["x86_64-linux"];
      devShells.systems = ["x86_64-linux"];
      packages.systems = ["x86_64-linux"];
    };
  };
}
