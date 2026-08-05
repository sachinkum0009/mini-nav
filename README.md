# mini-nav
Mini Nav for Robotics with Low Resources

## Commands

```bash
export TURTLEBOT3_MODEL=waffle

ros2 launch turtlebot3_gazebo turtlebot3_world.launch.py

cargo r -r --bin publisher

rviz2

ros2 service call /save_map example_interfaces/srv/SetBool "data: true"
```

## Commands to install Hiroz-Union

```bash
cargo install --git https://github.com/ZettaScaleLabs/hiroz.git hiroz-union
```
