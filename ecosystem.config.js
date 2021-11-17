module.exports =     // ecosystem.js
{
  "apps": [
    {
      "name": "HttpServer",
      "script": "./target/release/bundler-balancer",// name of the startup file
      "exec": "none",
      "exec_mode": "fork",  // to turn on cluster mode; defaults to 'fork' mode
      "env": {
        "PORT": "80", // the port on which the app should listen
        "kill_timeout": 120000
      }
    }
    ]
};