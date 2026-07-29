# Mini Nav Mapping

## Block Diagram

```mermaid
graph TD
  A[LiDAR Sensor] --> B[Mapping]
  C[Odometry] --> B
  B -> D[Map]
```
