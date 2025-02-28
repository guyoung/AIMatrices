import{m as U}from"./chunk-BAOP5US2-C9QlnglV-0987e145.js";import{A as j,m as H,X as I,a as Q,U as _,Y as q,G as J,d as u,$ as R,t as K,c as Z,D as tt,a6 as et,a8 as at,E as nt,a9 as y,aa as z,ab as rt}from"./index-be8a3043.js";import{X as it}from"./mermaid-parser.core-BgpndN-B-df4a9f1e.js";import{h as L}from"./arc-CgwJY7BF-b97ba03e.js";import{h as ot}from"./ordinal-DfAQgscy-dbc3c131.js";import"./index-7c5d0ad3.js";import"./index-3e281cae.js";import"./_baseUniq-v35O2o7d-331b5b2c.js";import"./_basePickBy-CyGFI_ex-b7b8d7ec.js";import"./clone-xDQJtMzt-f50c016f.js";import"./init-DjUOC4st-cac434d1.js";function lt(t,a){return a<t?-1:a>t?1:a>=t?0:NaN}function st(t){return t}function ct(){var t=st,a=lt,o=null,h=y(0),g=y(z),S=y(0);function i(e){var n,s=(e=rt(e)).length,c,$,x=0,p=new Array(s),r=new Array(s),m=+h.apply(this,arguments),A=Math.min(z,Math.max(-z,g.apply(this,arguments)-m)),f,C=Math.min(Math.abs(A)/s,S.apply(this,arguments)),D=C*(A<0?-1:1),d;for(n=0;n<s;++n)(d=r[p[n]=n]=+t(e[n],n,e))>0&&(x+=d);for(a!=null?p.sort(function(w,v){return a(r[w],r[v])}):o!=null&&p.sort(function(w,v){return o(e[w],e[v])}),n=0,$=x?(A-s*D)/x:0;n<s;++n,m=f)c=p[n],d=r[c],f=m+(d>0?d*$:0)+D,r[c]={data:e[c],index:n,value:d,startAngle:m,endAngle:f,padAngle:C};return r}return i.value=function(e){return arguments.length?(t=typeof e=="function"?e:y(+e),i):t},i.sortValues=function(e){return arguments.length?(a=e,o=null,i):a},i.sort=function(e){return arguments.length?(o=e,a=null,i):o},i.startAngle=function(e){return arguments.length?(h=typeof e=="function"?e:y(+e),i):h},i.endAngle=function(e){return arguments.length?(g=typeof e=="function"?e:y(+e),i):g},i.padAngle=function(e){return arguments.length?(S=typeof e=="function"?e:y(+e),i):S},i}var V=j.pie,E={sections:new Map,showData:!1,config:V},M=E.sections,W=E.showData,pt=structuredClone(V),ut=u(()=>structuredClone(pt),"getConfig"),dt=u(()=>{M=new Map,W=E.showData,K()},"clear"),gt=u(({label:t,value:a})=>{M.has(t)||(M.set(t,a),R.debug(`added new section: ${t}, with value: ${a}`))},"addSection"),mt=u(()=>M,"getSections"),ft=u(t=>{W=t},"setShowData"),ht=u(()=>W,"getShowData"),X={getConfig:ut,clear:dt,setDiagramTitle:H,getDiagramTitle:I,setAccTitle:Q,getAccTitle:_,setAccDescription:q,getAccDescription:J,addSection:gt,getSections:mt,setShowData:ft,getShowData:ht},xt=u((t,a)=>{U(t,a),a.setShowData(t.showData),t.sections.map(a.addSection)},"populateDb"),wt={parse:u(async t=>{const a=await it("pie",t);R.debug(a),xt(a,X)},"parse")},yt=u(t=>`
  .pieCircle{
    stroke: ${t.pieStrokeColor};
    stroke-width : ${t.pieStrokeWidth};
    opacity : ${t.pieOpacity};
  }
  .pieOuterCircle{
    stroke: ${t.pieOuterStrokeColor};
    stroke-width: ${t.pieOuterStrokeWidth};
    fill: none;
  }
  .pieTitleText {
    text-anchor: middle;
    font-size: ${t.pieTitleTextSize};
    fill: ${t.pieTitleTextColor};
    font-family: ${t.fontFamily};
  }
  .slice {
    font-family: ${t.fontFamily};
    fill: ${t.pieSectionTextColor};
    font-size:${t.pieSectionTextSize};
    // fill: white;
  }
  .legend text {
    fill: ${t.pieLegendTextColor};
    font-family: ${t.fontFamily};
    font-size: ${t.pieLegendTextSize};
  }
`,"getStyles"),St=yt,$t=u(t=>{const a=[...t.entries()].map(o=>({label:o[0],value:o[1]})).sort((o,h)=>h.value-o.value);return ct().value(o=>o.value)(a)},"createPieArcs"),At=u((t,a,o,h)=>{R.debug(`rendering pie chart
`+t);const g=h.db,S=Z(),i=tt(g.getConfig(),S.pie),e=40,n=18,s=4,c=450,$=c,x=et(a),p=x.append("g");p.attr("transform","translate("+$/2+","+c/2+")");const{themeVariables:r}=S;let[m]=at(r.pieOuterStrokeWidth);m??(m=2);const A=i.textPosition,f=Math.min($,c)/2-e,C=L().innerRadius(0).outerRadius(f),D=L().innerRadius(f*A).outerRadius(f*A);p.append("circle").attr("cx",0).attr("cy",0).attr("r",f+m/2).attr("class","pieOuterCircle");const d=g.getSections(),w=$t(d),v=[r.pie1,r.pie2,r.pie3,r.pie4,r.pie5,r.pie6,r.pie7,r.pie8,r.pie9,r.pie10,r.pie11,r.pie12],T=ot(v);p.selectAll("mySlices").data(w).enter().append("path").attr("d",C).attr("fill",l=>T(l.data.label)).attr("class","pieCircle");let B=0;d.forEach(l=>{B+=l}),p.selectAll("mySlices").data(w).enter().append("text").text(l=>(l.data.value/B*100).toFixed(0)+"%").attr("transform",l=>"translate("+D.centroid(l)+")").style("text-anchor","middle").attr("class","slice"),p.append("text").text(g.getDiagramTitle()).attr("x",0).attr("y",-(c-50)/2).attr("class","pieTitleText");const O=p.selectAll(".legend").data(T.domain()).enter().append("g").attr("class","legend").attr("transform",(l,b)=>{const k=n+s,N=k*T.domain().length/2,Y=12*n,P=b*k-N;return"translate("+Y+","+P+")"});O.append("rect").attr("width",n).attr("height",n).style("fill",T).style("stroke",T),O.data(w).append("text").attr("x",n+s).attr("y",n-s).text(l=>{const{label:b,value:k}=l.data;return g.getShowData()?`${b} [${k}]`:b});const G=Math.max(...O.selectAll("text").nodes().map(l=>(l==null?void 0:l.getBoundingClientRect().width)??0)),F=$+e+n+s+G;x.attr("viewBox",`0 0 ${F} ${c}`),nt(x,c,F,i.useMaxWidth)},"draw"),vt={draw:At},Bt={parser:wt,db:X,renderer:vt,styles:St};export{Bt as diagram};
