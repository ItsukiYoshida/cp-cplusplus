// ABC180-D
#include<bits/stdc++.h>
using namespace std;
#include<boost/multiprecision/cpp_int.hpp>
using namespace boost::multiprecision;
int main(){
  cpp_int x,y,a,b;
  cin>>x>>y>>a>>b;
  cpp_int ans=0;
  while(true){
    if(x*a>x+b||x*a>=y){
      break;
    }
    x*=a;
    ans++;
  }
  ans+=(y-1-x)/b;
  cout<<ans<<endl;
}
