import{j as L,k as i,l as M,m as ie,n as De,p as ht,q as ft,r as mt,a as X,t as Ae,v as ve,x as pt,y as R,z as gt,A as T,B as bt,C as yt,D as Me,E as ye,F as xt,G as we,H as wt,I as ke,J as S,V as kt,K as $t,L as _t,M as St,T as Ue,O as le,P as re,Q as de,R as zt,S as Ct,U as Rt,W as Tt,X as It,Y as Vt,Z as Dt,$ as Mt,a0 as Nt,a1 as Bt,a2 as Pe,o as J,c as $e,b as s,a3 as h,f as d,g as l,a4 as ue,w as y,h as N,N as B,a5 as At,a6 as K,a7 as Ut,a8 as Pt,a9 as Ft,aa as Et,ab as G,ac as Ht,ad as _e,ae as jt,af as Ot,ag as xe,ah as Lt,ai as Kt}from"./index-7c5d0ad3.js";import{u as Wt}from"./index-3e281cae.js";const Yt=L([i("slider",`
 display: block;
 padding: calc((var(--n-handle-size) - var(--n-rail-height)) / 2) 0;
 position: relative;
 z-index: 0;
 width: 100%;
 cursor: pointer;
 user-select: none;
 -webkit-user-select: none;
 `,[M("reverse",[i("slider-handles",[i("slider-handle-wrapper",`
 transform: translate(50%, -50%);
 `)]),i("slider-dots",[i("slider-dot",`
 transform: translateX(50%, -50%);
 `)]),M("vertical",[i("slider-handles",[i("slider-handle-wrapper",`
 transform: translate(-50%, -50%);
 `)]),i("slider-marks",[i("slider-mark",`
 transform: translateY(calc(-50% + var(--n-dot-height) / 2));
 `)]),i("slider-dots",[i("slider-dot",`
 transform: translateX(-50%) translateY(0);
 `)])])]),M("vertical",`
 box-sizing: content-box;
 padding: 0 calc((var(--n-handle-size) - var(--n-rail-height)) / 2);
 width: var(--n-rail-width-vertical);
 height: 100%;
 `,[i("slider-handles",`
 top: calc(var(--n-handle-size) / 2);
 right: 0;
 bottom: calc(var(--n-handle-size) / 2);
 left: 0;
 `,[i("slider-handle-wrapper",`
 top: unset;
 left: 50%;
 transform: translate(-50%, 50%);
 `)]),i("slider-rail",`
 height: 100%;
 `,[ie("fill",`
 top: unset;
 right: 0;
 bottom: unset;
 left: 0;
 `)]),M("with-mark",`
 width: var(--n-rail-width-vertical);
 margin: 0 32px 0 8px;
 `),i("slider-marks",`
 top: calc(var(--n-handle-size) / 2);
 right: unset;
 bottom: calc(var(--n-handle-size) / 2);
 left: 22px;
 font-size: var(--n-mark-font-size);
 `,[i("slider-mark",`
 transform: translateY(50%);
 white-space: nowrap;
 `)]),i("slider-dots",`
 top: calc(var(--n-handle-size) / 2);
 right: unset;
 bottom: calc(var(--n-handle-size) / 2);
 left: 50%;
 `,[i("slider-dot",`
 transform: translateX(-50%) translateY(50%);
 `)])]),M("disabled",`
 cursor: not-allowed;
 opacity: var(--n-opacity-disabled);
 `,[i("slider-handle",`
 cursor: not-allowed;
 `)]),M("with-mark",`
 width: 100%;
 margin: 8px 0 32px 0;
 `),L("&:hover",[i("slider-rail",{backgroundColor:"var(--n-rail-color-hover)"},[ie("fill",{backgroundColor:"var(--n-fill-color-hover)"})]),i("slider-handle",{boxShadow:"var(--n-handle-box-shadow-hover)"})]),M("active",[i("slider-rail",{backgroundColor:"var(--n-rail-color-hover)"},[ie("fill",{backgroundColor:"var(--n-fill-color-hover)"})]),i("slider-handle",{boxShadow:"var(--n-handle-box-shadow-hover)"})]),i("slider-marks",`
 position: absolute;
 top: 18px;
 left: calc(var(--n-handle-size) / 2);
 right: calc(var(--n-handle-size) / 2);
 `,[i("slider-mark",`
 position: absolute;
 transform: translateX(-50%);
 white-space: nowrap;
 `)]),i("slider-rail",`
 width: 100%;
 position: relative;
 height: var(--n-rail-height);
 background-color: var(--n-rail-color);
 transition: background-color .3s var(--n-bezier);
 border-radius: calc(var(--n-rail-height) / 2);
 `,[ie("fill",`
 position: absolute;
 top: 0;
 bottom: 0;
 border-radius: calc(var(--n-rail-height) / 2);
 transition: background-color .3s var(--n-bezier);
 background-color: var(--n-fill-color);
 `)]),i("slider-handles",`
 position: absolute;
 top: 0;
 right: calc(var(--n-handle-size) / 2);
 bottom: 0;
 left: calc(var(--n-handle-size) / 2);
 `,[i("slider-handle-wrapper",`
 outline: none;
 position: absolute;
 top: 50%;
 transform: translate(-50%, -50%);
 cursor: pointer;
 display: flex;
 `,[i("slider-handle",`
 height: var(--n-handle-size);
 width: var(--n-handle-size);
 border-radius: 50%;
 overflow: hidden;
 transition: box-shadow .2s var(--n-bezier), background-color .3s var(--n-bezier);
 background-color: var(--n-handle-color);
 box-shadow: var(--n-handle-box-shadow);
 `,[L("&:hover",`
 box-shadow: var(--n-handle-box-shadow-hover);
 `)]),L("&:focus",[i("slider-handle",`
 box-shadow: var(--n-handle-box-shadow-focus);
 `,[L("&:hover",`
 box-shadow: var(--n-handle-box-shadow-active);
 `)])])])]),i("slider-dots",`
 position: absolute;
 top: 50%;
 left: calc(var(--n-handle-size) / 2);
 right: calc(var(--n-handle-size) / 2);
 `,[M("transition-disabled",[i("slider-dot","transition: none;")]),i("slider-dot",`
 transition:
 border-color .3s var(--n-bezier),
 box-shadow .3s var(--n-bezier),
 background-color .3s var(--n-bezier);
 position: absolute;
 transform: translate(-50%, -50%);
 height: var(--n-dot-height);
 width: var(--n-dot-width);
 border-radius: var(--n-dot-border-radius);
 overflow: hidden;
 box-sizing: border-box;
 border: var(--n-dot-border);
 background-color: var(--n-dot-color);
 `,[M("active","border: var(--n-dot-border-active);")])])]),i("slider-handle-indicator",`
 font-size: var(--n-font-size);
 padding: 6px 10px;
 border-radius: var(--n-indicator-border-radius);
 color: var(--n-indicator-text-color);
 background-color: var(--n-indicator-color);
 box-shadow: var(--n-indicator-box-shadow);
 `,[De()]),i("slider-handle-indicator",`
 font-size: var(--n-font-size);
 padding: 6px 10px;
 border-radius: var(--n-indicator-border-radius);
 color: var(--n-indicator-text-color);
 background-color: var(--n-indicator-color);
 box-shadow: var(--n-indicator-box-shadow);
 `,[M("top",`
 margin-bottom: 12px;
 `),M("right",`
 margin-left: 12px;
 `),M("bottom",`
 margin-top: 12px;
 `),M("left",`
 margin-right: 12px;
 `),De()]),ht(i("slider",[i("slider-dot","background-color: var(--n-dot-color-modal);")])),ft(i("slider",[i("slider-dot","background-color: var(--n-dot-color-popover);")]))]);function Ne(n){return window.TouchEvent&&n instanceof window.TouchEvent}function Be(){const n=new Map,o=f=>v=>{n.set(f,v)};return mt(()=>{n.clear()}),[n,o]}const Gt=0,Jt=Object.assign(Object.assign({},ve.props),{to:ke.propTo,defaultValue:{type:[Number,Array],default:0},marks:Object,disabled:{type:Boolean,default:void 0},formatTooltip:Function,keyboard:{type:Boolean,default:!0},min:{type:Number,default:0},max:{type:Number,default:100},step:{type:[Number,String],default:1},range:Boolean,value:[Number,Array],placement:String,showTooltip:{type:Boolean,default:void 0},tooltip:{type:Boolean,default:!0},vertical:Boolean,reverse:Boolean,"onUpdate:value":[Function,Array],onUpdateValue:[Function,Array],onDragstart:[Function],onDragend:[Function]}),ce=X({name:"Slider",props:Jt,slots:Object,setup(n){const{mergedClsPrefixRef:o,namespaceRef:f,inlineThemeDisabled:v}=Ae(n),r=ve("Slider","-slider",Yt,pt,n,o),c=R(null),[z,k]=Be(),[p,C]=Be(),x=R(new Set),g=gt(n),{mergedDisabledRef:$}=g,H=T(()=>{const{step:e}=n;if(Number(e)<=0||e==="mark")return 0;const t=e.toString();let a=0;return t.includes(".")&&(a=t.length-t.indexOf(".")-1),a}),O=R(n.defaultValue),he=bt(n,"value"),q=yt(he,O),A=T(()=>{const{value:e}=q;return(n.range?e:[e]).map(Re)}),Q=T(()=>A.value.length>2),b=T(()=>n.placement===void 0?n.vertical?"right":"top":n.placement),_=T(()=>{const{marks:e}=n;return e?Object.keys(e).map(Number.parseFloat):null}),m=R(-1),E=R(-1),P=R(-1),U=R(!1),Z=R(!1),fe=T(()=>{const{vertical:e,reverse:t}=n;return e?t?"top":"bottom":t?"right":"left"}),Fe=T(()=>{if(Q.value)return;const e=A.value,t=ee(n.range?Math.min(...e):n.min),a=ee(n.range?Math.max(...e):e[0]),{value:u}=fe;return n.vertical?{[u]:`${t}%`,height:`${a-t}%`}:{[u]:`${t}%`,width:`${a-t}%`}}),Ee=T(()=>{const e=[],{marks:t}=n;if(t){const a=A.value.slice();a.sort((D,V)=>D-V);const{value:u}=fe,{value:w}=Q,{range:I}=n,F=w?()=>!1:D=>I?D>=a[0]&&D<=a[a.length-1]:D<=a[0];for(const D of Object.keys(t)){const V=Number(D);e.push({active:F(V),key:V,label:t[D],style:{[u]:`${ee(V)}%`}})}}return e});function He(e,t){const a=ee(e),{value:u}=fe;return{[u]:`${a}%`,zIndex:t===m.value?1:0}}function Se(e){return n.showTooltip||P.value===e||m.value===e&&U.value}function je(e){return U.value?!(m.value===e&&E.value===e):!0}function Oe(e){var t;~e&&(m.value=e,(t=z.get(e))===null||t===void 0||t.focus())}function Le(){p.forEach((e,t)=>{Se(t)&&e.syncPosition()})}function ze(e){const{"onUpdate:value":t,onUpdateValue:a}=n,{nTriggerFormInput:u,nTriggerFormChange:w}=g;a&&le(a,e),t&&le(t,e),O.value=e,u(),w()}function Ce(e){const{range:t}=n;if(t){if(Array.isArray(e)){const{value:a}=A;e.join()!==a.join()&&ze(e)}}else Array.isArray(e)||A.value[0]!==e&&ze(e)}function me(e,t){if(n.range){const a=A.value.slice();a.splice(t,1,e),Ce(a)}else Ce(e)}function pe(e,t,a){const u=a!==void 0;a||(a=e-t>0?1:-1);const w=_.value||[],{step:I}=n;if(I==="mark"){const V=te(e,w.concat(t),u?a:void 0);return V?V.value:t}if(I<=0)return t;const{value:F}=H;let D;if(u){const V=Number((t/I).toFixed(F)),j=Math.floor(V),ge=V>j?j:j-1,be=V<j?j:j+1;D=te(t,[Number((ge*I).toFixed(F)),Number((be*I).toFixed(F)),...w],a)}else{const V=We(e);D=te(e,[...w,V])}return D?Re(D.value):t}function Re(e){return Math.min(n.max,Math.max(n.min,e))}function ee(e){const{max:t,min:a}=n;return(e-a)/(t-a)*100}function Ke(e){const{max:t,min:a}=n;return a+(t-a)*e}function We(e){const{step:t,min:a}=n;if(Number(t)<=0||t==="mark")return e;const u=Math.round((e-a)/t)*t+a;return Number(u.toFixed(H.value))}function te(e,t=_.value,a){if(!(t!=null&&t.length))return null;let u=null,w=-1;for(;++w<t.length;){const I=t[w]-e,F=Math.abs(I);(a===void 0||I*a>0)&&(u===null||F<u.distance)&&(u={index:w,distance:F,value:t[w]})}return u}function Te(e){const t=c.value;if(!t)return;const a=Ne(e)?e.touches[0]:e,u=t.getBoundingClientRect();let w;return n.vertical?w=(u.bottom-a.clientY)/u.height:w=(a.clientX-u.left)/u.width,n.reverse&&(w=1-w),Ke(w)}function Ye(e){if($.value||!n.keyboard)return;const{vertical:t,reverse:a}=n;switch(e.key){case"ArrowUp":e.preventDefault(),ne(t&&a?-1:1);break;case"ArrowRight":e.preventDefault(),ne(!t&&a?-1:1);break;case"ArrowDown":e.preventDefault(),ne(t&&a?1:-1);break;case"ArrowLeft":e.preventDefault(),ne(!t&&a?1:-1);break}}function ne(e){const t=m.value;if(t===-1)return;const{step:a}=n,u=A.value[t],w=Number(a)<=0||a==="mark"?u:u+a*e;me(pe(w,u,e>0?1:-1),t)}function Ge(e){var t,a;if($.value||!Ne(e)&&e.button!==Gt)return;const u=Te(e);if(u===void 0)return;const w=A.value.slice(),I=n.range?(a=(t=te(u,w))===null||t===void 0?void 0:t.index)!==null&&a!==void 0?a:-1:0;I!==-1&&(e.preventDefault(),Oe(I),Je(),me(pe(u,A.value[I]),I))}function Je(){U.value||(U.value=!0,n.onDragstart&&le(n.onDragstart),re("touchend",document,oe),re("mouseup",document,oe),re("touchmove",document,se),re("mousemove",document,se))}function ae(){U.value&&(U.value=!1,n.onDragend&&le(n.onDragend),de("touchend",document,oe),de("mouseup",document,oe),de("touchmove",document,se),de("mousemove",document,se))}function se(e){const{value:t}=m;if(!U.value||t===-1){ae();return}const a=Te(e);a!==void 0&&me(pe(a,A.value[t]),t)}function oe(){ae()}function Xe(e){m.value=e,$.value||(P.value=e)}function qe(e){m.value===e&&(m.value=-1,ae()),P.value===e&&(P.value=-1)}function Qe(e){P.value=e}function Ze(e){P.value===e&&(P.value=-1)}Me(m,(e,t)=>void ye(()=>E.value=t)),Me(q,()=>{if(n.marks){if(Z.value)return;Z.value=!0,ye(()=>{Z.value=!1})}ye(Le)}),xt(()=>{ae()});const Ie=T(()=>{const{self:{markFontSize:e,railColor:t,railColorHover:a,fillColor:u,fillColorHover:w,handleColor:I,opacityDisabled:F,dotColor:D,dotColorModal:V,handleBoxShadow:j,handleBoxShadowHover:ge,handleBoxShadowActive:be,handleBoxShadowFocus:et,dotBorder:tt,dotBoxShadow:nt,railHeight:at,railWidthVertical:st,handleSize:ot,dotHeight:it,dotWidth:lt,dotBorderRadius:rt,fontSize:dt,dotBorderActive:ct,dotColorPopover:ut},common:{cubicBezierEaseInOut:vt}}=r.value;return{"--n-bezier":vt,"--n-dot-border":tt,"--n-dot-border-active":ct,"--n-dot-border-radius":rt,"--n-dot-box-shadow":nt,"--n-dot-color":D,"--n-dot-color-modal":V,"--n-dot-color-popover":ut,"--n-dot-height":it,"--n-dot-width":lt,"--n-fill-color":u,"--n-fill-color-hover":w,"--n-font-size":dt,"--n-handle-box-shadow":j,"--n-handle-box-shadow-active":be,"--n-handle-box-shadow-focus":et,"--n-handle-box-shadow-hover":ge,"--n-handle-color":I,"--n-handle-size":ot,"--n-opacity-disabled":F,"--n-rail-color":t,"--n-rail-color-hover":a,"--n-rail-height":at,"--n-rail-width-vertical":st,"--n-mark-font-size":e}}),W=v?we("slider",void 0,Ie,n):void 0,Ve=T(()=>{const{self:{fontSize:e,indicatorColor:t,indicatorBoxShadow:a,indicatorTextColor:u,indicatorBorderRadius:w}}=r.value;return{"--n-font-size":e,"--n-indicator-border-radius":w,"--n-indicator-box-shadow":a,"--n-indicator-color":t,"--n-indicator-text-color":u}}),Y=v?we("slider-indicator",void 0,Ve,n):void 0;return{mergedClsPrefix:o,namespace:f,uncontrolledValue:O,mergedValue:q,mergedDisabled:$,mergedPlacement:b,isMounted:wt(),adjustedTo:ke(n),dotTransitionDisabled:Z,markInfos:Ee,isShowTooltip:Se,shouldKeepTooltipTransition:je,handleRailRef:c,setHandleRefs:k,setFollowerRefs:C,fillStyle:Fe,getHandleStyle:He,activeIndex:m,arrifiedValues:A,followerEnabledIndexSet:x,handleRailMouseDown:Ge,handleHandleFocus:Xe,handleHandleBlur:qe,handleHandleMouseEnter:Qe,handleHandleMouseLeave:Ze,handleRailKeyDown:Ye,indicatorCssVars:v?void 0:Ve,indicatorThemeClass:Y==null?void 0:Y.themeClass,indicatorOnRender:Y==null?void 0:Y.onRender,cssVars:v?void 0:Ie,themeClass:W==null?void 0:W.themeClass,onRender:W==null?void 0:W.onRender}},render(){var n;const{mergedClsPrefix:o,themeClass:f,formatTooltip:v}=this;return(n=this.onRender)===null||n===void 0||n.call(this),S("div",{class:[`${o}-slider`,f,{[`${o}-slider--disabled`]:this.mergedDisabled,[`${o}-slider--active`]:this.activeIndex!==-1,[`${o}-slider--with-mark`]:this.marks,[`${o}-slider--vertical`]:this.vertical,[`${o}-slider--reverse`]:this.reverse}],style:this.cssVars,onKeydown:this.handleRailKeyDown,onMousedown:this.handleRailMouseDown,onTouchstart:this.handleRailMouseDown},S("div",{class:`${o}-slider-rail`},S("div",{class:`${o}-slider-rail__fill`,style:this.fillStyle}),this.marks?S("div",{class:[`${o}-slider-dots`,this.dotTransitionDisabled&&`${o}-slider-dots--transition-disabled`]},this.markInfos.map(r=>S("div",{key:r.key,class:[`${o}-slider-dot`,{[`${o}-slider-dot--active`]:r.active}],style:r.style}))):null,S("div",{ref:"handleRailRef",class:`${o}-slider-handles`},this.arrifiedValues.map((r,c)=>{const z=this.isShowTooltip(c);return S(kt,null,{default:()=>[S($t,null,{default:()=>S("div",{ref:this.setHandleRefs(c),class:`${o}-slider-handle-wrapper`,tabindex:this.mergedDisabled?-1:0,role:"slider","aria-valuenow":r,"aria-valuemin":this.min,"aria-valuemax":this.max,"aria-orientation":this.vertical?"vertical":"horizontal","aria-disabled":this.disabled,style:this.getHandleStyle(r,c),onFocus:()=>{this.handleHandleFocus(c)},onBlur:()=>{this.handleHandleBlur(c)},onMouseenter:()=>{this.handleHandleMouseEnter(c)},onMouseleave:()=>{this.handleHandleMouseLeave(c)}},_t(this.$slots.thumb,()=>[S("div",{class:`${o}-slider-handle`})]))}),this.tooltip&&S(St,{ref:this.setFollowerRefs(c),show:z,to:this.adjustedTo,enabled:this.showTooltip&&!this.range||this.followerEnabledIndexSet.has(c),teleportDisabled:this.adjustedTo===ke.tdkey,placement:this.mergedPlacement,containerClass:this.namespace},{default:()=>S(Ue,{name:"fade-in-scale-up-transition",appear:this.isMounted,css:this.shouldKeepTooltipTransition(c),onEnter:()=>{this.followerEnabledIndexSet.add(c)},onAfterLeave:()=>{this.followerEnabledIndexSet.delete(c)}},{default:()=>{var k;return z?((k=this.indicatorOnRender)===null||k===void 0||k.call(this),S("div",{class:[`${o}-slider-handle-indicator`,this.indicatorThemeClass,`${o}-slider-handle-indicator--${this.mergedPlacement}`],style:this.indicatorCssVars},typeof v=="function"?v(r):r)):null}})})]})})),this.marks?S("div",{class:`${o}-slider-marks`},this.markInfos.map(r=>S("div",{key:r.key,class:`${o}-slider-mark`,style:r.style},typeof r.label=="function"?r.label():r.label))):null))}}),Xt=L([L("@keyframes spin-rotate",`
 from {
 transform: rotate(0);
 }
 to {
 transform: rotate(360deg);
 }
 `),i("spin-container",`
 position: relative;
 `,[i("spin-body",`
 position: absolute;
 top: 50%;
 left: 50%;
 transform: translateX(-50%) translateY(-50%);
 `,[zt()])]),i("spin-body",`
 display: inline-flex;
 align-items: center;
 justify-content: center;
 flex-direction: column;
 `),i("spin",`
 display: inline-flex;
 height: var(--n-size);
 width: var(--n-size);
 font-size: var(--n-size);
 color: var(--n-color);
 `,[M("rotate",`
 animation: spin-rotate 2s linear infinite;
 `)]),i("spin-description",`
 display: inline-block;
 font-size: var(--n-font-size);
 color: var(--n-text-color);
 transition: color .3s var(--n-bezier);
 margin-top: 8px;
 `),i("spin-content",`
 opacity: 1;
 transition: opacity .3s var(--n-bezier);
 pointer-events: all;
 `,[M("spinning",`
 user-select: none;
 -webkit-user-select: none;
 pointer-events: none;
 opacity: var(--n-opacity-spinning);
 `)])]),qt={small:20,medium:18,large:16},Qt=Object.assign(Object.assign({},ve.props),{contentClass:String,contentStyle:[Object,String],description:String,stroke:String,size:{type:[String,Number],default:"medium"},show:{type:Boolean,default:!0},strokeWidth:Number,rotate:{type:Boolean,default:!0},spinning:{type:Boolean,validator:()=>!0,default:void 0},delay:Number}),Zt=X({name:"Spin",props:Qt,slots:Object,setup(n){const{mergedClsPrefixRef:o,inlineThemeDisabled:f}=Ae(n),v=ve("Spin","-spin",Xt,Ct,n,o),r=T(()=>{const{size:p}=n,{common:{cubicBezierEaseInOut:C},self:x}=v.value,{opacitySpinning:g,color:$,textColor:H}=x,O=typeof p=="number"?Rt(p):x[Tt("size",p)];return{"--n-bezier":C,"--n-opacity-spinning":g,"--n-size":O,"--n-color":$,"--n-text-color":H}}),c=f?we("spin",T(()=>{const{size:p}=n;return typeof p=="number"?String(p):p[0]}),r,n):void 0,z=It(n,["spinning","show"]),k=R(!1);return Vt(p=>{let C;if(z.value){const{delay:x}=n;if(x){C=window.setTimeout(()=>{k.value=!0},x),p(()=>{clearTimeout(C)});return}}k.value=z.value}),{mergedClsPrefix:o,active:k,mergedStrokeWidth:T(()=>{const{strokeWidth:p}=n;if(p!==void 0)return p;const{size:C}=n;return qt[typeof C=="number"?"medium":C]}),cssVars:f?void 0:r,themeClass:c==null?void 0:c.themeClass,onRender:c==null?void 0:c.onRender}},render(){var n,o;const{$slots:f,mergedClsPrefix:v,description:r}=this,c=f.icon&&this.rotate,z=(r||f.description)&&S("div",{class:`${v}-spin-description`},r||((n=f.description)===null||n===void 0?void 0:n.call(f))),k=f.icon?S("div",{class:[`${v}-spin-body`,this.themeClass]},S("div",{class:[`${v}-spin`,c&&`${v}-spin--rotate`],style:f.default?"":this.cssVars},f.icon()),z):S("div",{class:[`${v}-spin-body`,this.themeClass]},S(Dt,{clsPrefix:v,style:f.default?"":this.cssVars,stroke:this.stroke,"stroke-width":this.mergedStrokeWidth,class:`${v}-spin`}),z);return(o=this.onRender)===null||o===void 0||o.call(this),f.default?S("div",{class:[`${v}-spin-container`,this.themeClass],style:this.cssVars},S("div",{class:[`${v}-spin-content`,this.active&&`${v}-spin-content--spinning`,this.contentClass],style:this.contentStyle},f),S(Ue,{name:"fade-in-transition"},{default:()=>this.active?k:null})):k}});function en(){const n=new Date,o=n.getDate(),f=n.getMonth()+1;return`${n.getFullYear()}-${f}-${o}`}const tn={class:"p-4 space-y-5 min-h-[200px]"},nn={class:"space-y-6"},an={class:"flex items-center space-x-4"},sn={class:"flex-shrink-0 w-[100px]"},on={class:"flex-1"},ln={class:"flex items-center space-x-4"},rn={class:"flex-shrink-0 w-[100px]"},dn={class:"w-[200px]"},cn={class:"flex items-center space-x-4"},un={class:"flex-shrink-0 w-[100px]"},vn={class:"flex-1"},hn={class:"flex-shrink-0 w-[100px]"},fn={class:"flex flex-wrap items-center gap-4"},mn={class:"flex items-center space-x-4"},pn={class:"flex-shrink-0 w-[100px]"},gn={class:"flex flex-wrap items-center gap-4"},bn={class:"flex items-center space-x-4"},yn={class:"flex-shrink-0 w-[100px]"},xn={class:"flex flex-wrap items-center gap-4"},wn={class:"flex items-center space-x-4"},kn={class:"flex-shrink-0 w-[100px]"},$n=X({__name:"General",setup(n){const o=Mt(),f=Nt(),{isMobile:v}=Bt(),r=Pe(),c=T(()=>o.theme),z=T(()=>f.userInfo),k=R(z.value.avatar??""),p=R(z.value.name??""),C=R(z.value.description??""),x=T({get(){return o.language},set(b){o.setLanguage(b)}}),g=[{label:"Auto",key:"auto",icon:"ri:contrast-line"},{label:"Light",key:"light",icon:"ri:sun-foggy-line"},{label:"Dark",key:"dark",icon:"ri:moon-foggy-line"}],$=[{label:"English",key:"en-US",value:"en-US"},{label:"简体中文",key:"zh-CN",value:"zh-CN"},{label:"繁體中文",key:"zh-TW",value:"zh-TW"},{label:"日本語",key:"ja-JP",value:"ja-JP"},{label:"한국어",key:"ko-KR",value:"ko-KR"},{label:"Русский язык",key:"ru-RU",value:"ru-RU"},{label:"Việt nam",key:"vi-VN",value:"vi-VN"}];function H(b){f.updateUserInfo(b),r.success(G("common.success"))}function O(){f.resetUserInfo(),r.success(G("common.success")),window.location.reload()}function he(){const b=en(),_=localStorage.getItem("chatStorage")||"{}",m=JSON.stringify(JSON.parse(_),null,2),E=new Blob([m],{type:"application/json"}),P=URL.createObjectURL(E),U=document.createElement("a");U.href=P,U.download=`chat-store_${b}.json`,document.body.appendChild(U),U.click(),document.body.removeChild(U)}function q(b){const _=b.target;if(!_||!_.files)return;const m=_.files[0];if(!m)return;const E=new FileReader;E.onload=()=>{try{const P=JSON.parse(E.result);localStorage.setItem("chatStorage",JSON.stringify(P)),r.success(G("common.success")),location.reload()}catch{r.error(G("common.invalidFileFormat"))}},E.readAsText(m)}function A(){localStorage.removeItem("chatStorage"),location.reload()}function Q(){const b=document.getElementById("fileInput");b&&b.click()}return(b,_)=>(J(),$e("div",tn,[s("div",nn,[s("div",an,[s("span",sn,h(b.$t("setting.avatarLink")),1),s("div",on,[d(l(ue),{value:k.value,"onUpdate:value":_[0]||(_[0]=m=>k.value=m),placeholder:""},null,8,["value"])]),d(l(B),{size:"tiny",text:"",type:"primary",onClick:_[1]||(_[1]=m=>H({avatar:k.value}))},{default:y(()=>[N(h(b.$t("common.save")),1)]),_:1})]),s("div",ln,[s("span",rn,h(b.$t("setting.name")),1),s("div",dn,[d(l(ue),{value:p.value,"onUpdate:value":_[2]||(_[2]=m=>p.value=m),placeholder:""},null,8,["value"])]),d(l(B),{size:"tiny",text:"",type:"primary",onClick:_[3]||(_[3]=m=>H({name:p.value}))},{default:y(()=>[N(h(b.$t("common.save")),1)]),_:1})]),s("div",cn,[s("span",un,h(b.$t("setting.description")),1),s("div",vn,[d(l(ue),{value:C.value,"onUpdate:value":_[4]||(_[4]=m=>C.value=m),placeholder:""},null,8,["value"])]),d(l(B),{size:"tiny",text:"",type:"primary",onClick:_[5]||(_[5]=m=>H({description:C.value}))},{default:y(()=>[N(h(b.$t("common.save")),1)]),_:1})]),s("div",{class:At(["flex items-center space-x-4",l(v)&&"items-start"])},[s("span",hn,h(b.$t("setting.chatHistory")),1),s("div",fn,[d(l(B),{size:"small",onClick:he},{icon:y(()=>[d(l(K),{icon:"ri:download-2-fill"})]),default:y(()=>[N(" "+h(b.$t("common.export")),1)]),_:1}),s("input",{id:"fileInput",type:"file",style:{display:"none"},onChange:q},null,32),d(l(B),{size:"small",onClick:Q},{icon:y(()=>[d(l(K),{icon:"ri:upload-2-fill"})]),default:y(()=>[N(" "+h(b.$t("common.import")),1)]),_:1}),d(l(Ut),{placement:"bottom",onPositiveClick:A},{trigger:y(()=>[d(l(B),{size:"small"},{icon:y(()=>[d(l(K),{icon:"ri:close-circle-line"})]),default:y(()=>[N(" "+h(b.$t("common.clear")),1)]),_:1})]),default:y(()=>[N(" "+h(b.$t("chat.clearHistoryConfirm")),1)]),_:1})])],2),s("div",mn,[s("span",pn,h(b.$t("setting.theme")),1),s("div",gn,[(J(),$e(Pt,null,Ft(g,m=>d(l(B),{key:m.key,size:"small",type:m.key===c.value?"primary":void 0,onClick:E=>l(o).setTheme(m.key)},{icon:y(()=>[d(l(K),{icon:m.icon},null,8,["icon"])]),_:2},1032,["type","onClick"])),64))])]),s("div",bn,[s("span",yn,h(b.$t("setting.language")),1),s("div",xn,[d(l(Et),{style:{width:"140px"},value:x.value,options:$,onUpdateValue:_[6]||(_[6]=m=>l(o).setLanguage(m))},null,8,["value"])])]),s("div",wn,[s("span",kn,h(b.$t("setting.resetUserInfo")),1),d(l(B),{size:"small",onClick:O},{default:y(()=>[N(h(b.$t("common.reset")),1)]),_:1})])])]))}}),_n={class:"p-4 space-y-5 min-h-[200px]"},Sn={class:"space-y-6"},zn={class:"flex items-center space-x-4"},Cn={class:"flex-shrink-0 w-[120px]"},Rn={class:"flex-1"},Tn={class:"flex items-center space-x-4"},In={class:"flex-shrink-0 w-[120px]"},Vn={class:"flex-1"},Dn={class:"flex items-center space-x-4"},Mn={class:"flex-shrink-0 w-[120px]"},Nn={class:"flex-1"},Bn={class:"flex items-center space-x-4"},An={class:"flex-shrink-0 w-[120px]"},Un={class:"flex-1"},Pn={class:"flex items-center space-x-4"},Fn={class:"flex-shrink-0 w-[120px]"},En={class:"flex-1"},Hn={class:"flex items-center space-x-4"},jn=X({__name:"Advanced",setup(n){const o=Wt(),f=Pe(),v=R(o.systemMessage??""),r=R(o.multiRoundChat??1),c=R(o.maxTokens??1e3),z=R(o.temperature??.5),k=R(o.top_p??1);function p(x){o.updateSetting(x),f.success(G("common.success"))}function C(){o.resetSetting(),f.success(G("common.success")),window.location.reload()}return(x,g)=>(J(),$e("div",_n,[s("div",Sn,[s("div",zn,[s("span",Cn,h(x.$t("setting.role")),1),s("div",Rn,[d(l(ue),{value:v.value,"onUpdate:value":g[0]||(g[0]=$=>v.value=$),type:"textarea",autosize:{minRows:1,maxRows:4}},null,8,["value"])]),d(l(B),{size:"tiny",text:"",type:"primary",onClick:g[1]||(g[1]=$=>p({systemMessage:v.value}))},{default:y(()=>[N(h(x.$t("common.save")),1)]),_:1})]),s("div",Tn,[s("span",In,h(x.$t("setting.multiRoundChat")),1),s("div",Vn,[d(l(ce),{value:r.value,"onUpdate:value":g[2]||(g[2]=$=>r.value=$),max:20,min:1,step:1},null,8,["value"])]),s("span",null,h(r.value),1),d(l(B),{size:"tiny",text:"",type:"primary",onClick:g[3]||(g[3]=$=>p({multiRoundChat:r.value}))},{default:y(()=>[N(h(x.$t("common.save")),1)]),_:1})]),s("div",Dn,[s("span",Mn,h(x.$t("setting.maxTokens")),1),s("div",Nn,[d(l(ce),{value:c.value,"onUpdate:value":g[4]||(g[4]=$=>c.value=$),max:10240,min:100,step:1},null,8,["value"])]),s("span",null,h(c.value),1),d(l(B),{size:"tiny",text:"",type:"primary",onClick:g[5]||(g[5]=$=>p({maxTokens:c.value}))},{default:y(()=>[N(h(x.$t("common.save")),1)]),_:1})]),s("div",Bn,[s("span",An,h(x.$t("setting.temperature")),1),s("div",Un,[d(l(ce),{value:z.value,"onUpdate:value":g[6]||(g[6]=$=>z.value=$),max:2,min:0,step:.1},null,8,["value"])]),s("span",null,h(z.value),1),d(l(B),{size:"tiny",text:"",type:"primary",onClick:g[7]||(g[7]=$=>p({temperature:z.value}))},{default:y(()=>[N(h(x.$t("common.save")),1)]),_:1})]),s("div",Pn,[s("span",Fn,h(x.$t("setting.top_p")),1),s("div",En,[d(l(ce),{value:k.value,"onUpdate:value":g[8]||(g[8]=$=>k.value=$),max:1,min:0,step:.1},null,8,["value"])]),s("span",null,h(k.value),1),d(l(B),{size:"tiny",text:"",type:"primary",onClick:g[9]||(g[9]=$=>p({top_p:k.value}))},{default:y(()=>[N(h(x.$t("common.save")),1)]),_:1})]),s("div",Hn,[g[10]||(g[10]=s("span",{class:"flex-shrink-0 w-[120px]"}," ",-1)),d(l(B),{size:"small",onClick:C},{default:y(()=>[N(h(x.$t("common.reset")),1)]),_:1})])])]))}}),On=X({__name:"About",setup(n){const o=R(!1),f=R();async function v(){try{o.value=!0;const r={};f.value=r}finally{o.value=!1}}return Ht(()=>{v()}),(r,c)=>(J(),_e(l(Zt),{show:o.value},{default:y(()=>c[0]||(c[0]=[s("div",{class:"p-4 space-y-4"},[s("h2",{class:"text-xl font-bold"}," AIMatrices(DeepSeek) "),s("div",{class:"p-2 space-y-2 rounded-md bg-neutral-100 dark:bg-neutral-700"}," AIMatrices is a lightweight, high-performance, scalable, and open source AI application rapid building platform designed to provide developers with an efficient and convenient AI application development experience. "),s("div",{class:"p-2 space-y-2 rounded-md bg-neutral-100 dark:bg-neutral-700"},[N(" Site: "),s("a",{href:"https://github.com/guyoung/AIMatrices",target:"_blank"},"https://github.com/guyoung/AIMatrices")])],-1)])),_:1},8,["show"]))}}),Ln={class:"ml-2"},Kn={class:"min-h-[100px]"},Wn={class:"ml-2"},Yn={class:"min-h-[100px]"},Gn={class:"ml-2"},qn=X({__name:"index",props:{visible:{type:Boolean}},emits:["update:visible"],setup(n,{emit:o}){const f=n,v=o,r=jt(),c=T(()=>!!r.isChatGPTAPI),z=R("General"),k=T({get(){return f.visible},set(p){v("update:visible",p)}});return(p,C)=>(J(),_e(l(Kt),{show:k.value,"onUpdate:show":C[1]||(C[1]=x=>k.value=x),"auto-focus":!1,preset:"card",style:{width:"95%","max-width":"640px"}},{default:y(()=>[s("div",null,[d(l(Ot),{value:z.value,"onUpdate:value":C[0]||(C[0]=x=>z.value=x),type:"line",animated:""},{default:y(()=>[d(l(xe),{name:"General",tab:"General"},{tab:y(()=>[d(l(K),{class:"text-lg",icon:"ri:file-user-line"}),s("span",Ln,h(p.$t("setting.general")),1)]),default:y(()=>[s("div",Kn,[d($n)])]),_:1}),c.value?(J(),_e(l(xe),{key:0,name:"Advanced",tab:"Advanced"},{tab:y(()=>[d(l(K),{class:"text-lg",icon:"ri:equalizer-line"}),s("span",Wn,h(p.$t("setting.advanced")),1)]),default:y(()=>[s("div",Yn,[d(jn)])]),_:1})):Lt("",!0),d(l(xe),{name:"About",tab:"About"},{tab:y(()=>[d(l(K),{class:"text-lg",icon:"ri:list-settings-line"}),s("span",Gn,h(p.$t("setting.about")),1)]),default:y(()=>[d(On)]),_:1})]),_:1},8,["value"])])]),_:1},8,["show"]))}});export{qn as default};
