# How to Debug ImageMagick

Let's take `convert` command as an example. 


### `-debug flag`

You can see debug log with `-debug all` options.

```
convert -debug all -scale 1024x1024 sample.tiff out.png
```




### `-list resource`

You can see resource limits for imageMagick (configured by policy.yml)

```
convert -list resource


Resource limits:
  Width: 1.1529215EP
  Height: 1.1529215EP
  List length: unlimited
  Area: 380MP
  Memory: 12GiB
  Map: 1GiB
  Disk: 0B
  File: 786432
  Thread: 10
  Throttle: 0
  Time: unlimited
```


