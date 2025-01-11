# 1.开发环境FAQ
## 1.1.为什么RA插件对有的项目不生效?
因为Cargo会忽略不在workspace-memebers中的文件夹,因此RA也不起作用.

## 1.2.同一个文件夹下同时测试stable和兼容nightly的代码示例处理?
通过将workspace-members的注释开启,来关闭不相关的包.

## 1.3.Remote提示编译失败如何处理
到服务器编译一次,看看失败原因并修复.