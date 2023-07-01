hittable_list里面不知道怎么解决tmp_rec没有初值的问题，所以参考了@JolyneFr的代码（返回Option<T>）.

关于bvh：场景为bouncing sphere，＃23没加bvh，build and upload用时3m56s；＃29加了bvh，build and upload用时1m41s.