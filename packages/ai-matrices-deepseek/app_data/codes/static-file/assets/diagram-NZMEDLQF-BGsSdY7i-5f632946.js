import{m as B}from"./chunk-BAOP5US2-C9QlnglV-59ce7fe1.js";import{A as S,a as F,U as P,m as W,X as z,G as T,Y,d as i,$ as x,D as $,F as v,t as D,a6 as E,E as X}from"./index-8de624c0.js";import{X as A}from"./mermaid-parser.core-BgpndN-B-06136a96.js";import"./index-5c534fb6.js";import"./index-ae1fd19d.js";import"./_baseUniq-v35O2o7d-4ef813cf.js";import"./_basePickBy-CyGFI_ex-ffe67e2e.js";import"./clone-xDQJtMzt-5bc95e6e.js";var w={packet:[]},u=structuredClone(w),H=S.packet,R=i(()=>{const t=$({...H,...v().packet});return t.showBits&&(t.paddingY+=10),t},"getConfig"),G=i(()=>u.packet,"getPacket"),L=i(t=>{t.length>0&&u.packet.push(t)},"pushWord"),U=i(()=>{D(),u=structuredClone(w)},"clear"),m={pushWord:L,getPacket:G,getConfig:R,clear:U,setAccTitle:F,getAccTitle:P,setDiagramTitle:W,getDiagramTitle:z,getAccDescription:T,setAccDescription:Y},j=1e4,I=i(t=>{B(t,m);let e=-1,o=[],s=1;const{bitsPerRow:n}=m.getConfig();for(let{start:a,end:r,label:p}of t.blocks){if(r&&r<a)throw new Error(`Packet block ${a} - ${r} is invalid. End must be greater than start.`);if(a!==e+1)throw new Error(`Packet block ${a} - ${r??a} is not contiguous. It should start from ${e+1}.`);for(e=r??a,x.debug(`Packet block ${a} - ${e} with label ${p}`);o.length<=n+1&&m.getPacket().length<j;){const[h,c]=M({start:a,end:r,label:p},s,n);if(o.push(h),h.end+1===s*n&&(m.pushWord(o),o=[],s++),!c)break;({start:a,end:r,label:p}=c)}}m.pushWord(o)},"populate"),M=i((t,e,o)=>{if(t.end===void 0&&(t.end=t.start),t.start>t.end)throw new Error(`Block start ${t.start} is greater than block end ${t.end}.`);return t.end+1<=e*o?[t,void 0]:[{start:t.start,end:e*o-1,label:t.label},{start:e*o,end:t.end,label:t.label}]},"getNextFittingBlock"),q={parse:i(async t=>{const e=await A("packet",t);x.debug(e),I(e)},"parse")},J=i((t,e,o,s)=>{const n=s.db,a=n.getConfig(),{rowHeight:r,paddingY:p,bitWidth:h,bitsPerRow:c}=a,f=n.getPacket(),l=n.getDiagramTitle(),k=r+p,d=k*(f.length+1)-(l?0:r),b=h*c+2,g=E(e);g.attr("viewbox",`0 0 ${b} ${d}`),X(g,d,b,a.useMaxWidth);for(const[y,C]of f.entries())K(g,C,y,a);g.append("text").text(l).attr("x",b/2).attr("y",d-k/2).attr("dominant-baseline","middle").attr("text-anchor","middle").attr("class","packetTitle")},"draw"),K=i((t,e,o,{rowHeight:s,paddingX:n,paddingY:a,bitWidth:r,bitsPerRow:p,showBits:h})=>{const c=t.append("g"),f=o*(s+a)+a;for(const l of e){const k=l.start%p*r+1,d=(l.end-l.start+1)*r-n;if(c.append("rect").attr("x",k).attr("y",f).attr("width",d).attr("height",s).attr("class","packetBlock"),c.append("text").attr("x",k+d/2).attr("y",f+s/2).attr("class","packetLabel").attr("dominant-baseline","middle").attr("text-anchor","middle").text(l.label),!h)continue;const b=l.end===l.start,g=f-2;c.append("text").attr("x",k+(b?d/2:0)).attr("y",g).attr("class","packetByte start").attr("dominant-baseline","auto").attr("text-anchor",b?"middle":"start").text(l.start),b||c.append("text").attr("x",k+d).attr("y",g).attr("class","packetByte end").attr("dominant-baseline","auto").attr("text-anchor","end").text(l.end)}},"drawWord"),N={draw:J},O={byteFontSize:"10px",startByteColor:"black",endByteColor:"black",labelColor:"black",labelFontSize:"12px",titleColor:"black",titleFontSize:"14px",blockStrokeColor:"black",blockStrokeWidth:"1",blockFillColor:"#efefef"},V=i(({packet:t}={})=>{const e=$(O,t);return`
	.packetByte {
		font-size: ${e.byteFontSize};
	}
	.packetByte.start {
		fill: ${e.startByteColor};
	}
	.packetByte.end {
		fill: ${e.endByteColor};
	}
	.packetLabel {
		fill: ${e.labelColor};
		font-size: ${e.labelFontSize};
	}
	.packetTitle {
		fill: ${e.titleColor};
		font-size: ${e.titleFontSize};
	}
	.packetBlock {
		stroke: ${e.blockStrokeColor};
		stroke-width: ${e.blockStrokeWidth};
		fill: ${e.blockFillColor};
	}
	`},"styles"),lt={parser:q,db:m,renderer:N,styles:V};export{lt as diagram};
