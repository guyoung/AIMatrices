import{c as L,a as i,ae as I,af as ie,f as Ie,ag as ht,ah as ft,ai as mt,d as G,u as Ue,g as ue,aj as pt,r as z,b as gt,h as R,t as bt,e as yt,ak as Me,Z as be,al as xt,j as we,k as wt,l as ke,m as _,V as kt,n as $t,am as _t,p as St,T as Ae,z as le,an as re,ao as de,ap as zt,aq as Ct,ar as Rt,as as Tt,at as Vt,au as Dt,av as It,a1 as Mt,A as Nt,K as Bt,S as Pe,B as Y,C as $e,J as o,R as g,U as u,D as r,N as ce,X as y,a9 as B,aa as U,Q as Ut,_ as K,aw as At,I as Pt,a8 as Ft,a2 as jt,L as X,M as Et,F as _e,a0 as Ht,ax as Ot,ay as ye,W as Lt,az as Kt}from"./index-700aa357.js";import{u as Wt}from"./index-75608336.js";const Jt=L([i("slider",`
 display: block;
 padding: calc((var(--n-handle-size) - var(--n-rail-height)) / 2) 0;
 position: relative;
 z-index: 0;
 width: 100%;
 cursor: pointer;
 user-select: none;
 -webkit-user-select: none;
 `,[I("reverse",[i("slider-handles",[i("slider-handle-wrapper",`
 transform: translate(50%, -50%);
 `)]),i("slider-dots",[i("slider-dot",`
 transform: translateX(50%, -50%);
 `)]),I("vertical",[i("slider-handles",[i("slider-handle-wrapper",`
 transform: translate(-50%, -50%);
 `)]),i("slider-marks",[i("slider-mark",`
 transform: translateY(calc(-50% + var(--n-dot-height) / 2));
 `)]),i("slider-dots",[i("slider-dot",`
 transform: translateX(-50%) translateY(0);
 `)])])]),I("vertical",`
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
 `)]),I("with-mark",`
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
 `)])]),I("disabled",`
 cursor: not-allowed;
 opacity: var(--n-opacity-disabled);
 `,[i("slider-handle",`
 cursor: not-allowed;
 `)]),I("with-mark",`
 width: 100%;
 margin: 8px 0 32px 0;
 `),L("&:hover",[i("slider-rail",{backgroundColor:"var(--n-rail-color-hover)"},[ie("fill",{backgroundColor:"var(--n-fill-color-hover)"})]),i("slider-handle",{boxShadow:"var(--n-handle-box-shadow-hover)"})]),I("active",[i("slider-rail",{backgroundColor:"var(--n-rail-color-hover)"},[ie("fill",{backgroundColor:"var(--n-fill-color-hover)"})]),i("slider-handle",{boxShadow:"var(--n-handle-box-shadow-hover)"})]),i("slider-marks",`
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
 `,[I("transition-disabled",[i("slider-dot","transition: none;")]),i("slider-dot",`
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
 `,[I("active","border: var(--n-dot-border-active);")])])]),i("slider-handle-indicator",`
 font-size: var(--n-font-size);
 padding: 6px 10px;
 border-radius: var(--n-indicator-border-radius);
 color: var(--n-indicator-text-color);
 background-color: var(--n-indicator-color);
 box-shadow: var(--n-indicator-box-shadow);
 `,[Ie()]),i("slider-handle-indicator",`
 font-size: var(--n-font-size);
 padding: 6px 10px;
 border-radius: var(--n-indicator-border-radius);
 color: var(--n-indicator-text-color);
 background-color: var(--n-indicator-color);
 box-shadow: var(--n-indicator-box-shadow);
 `,[I("top",`
 margin-bottom: 12px;
 `),I("right",`
 margin-left: 12px;
 `),I("bottom",`
 margin-top: 12px;
 `),I("left",`
 margin-right: 12px;
 `),Ie()]),ht(i("slider",[i("slider-dot","background-color: var(--n-dot-color-modal);")])),ft(i("slider",[i("slider-dot","background-color: var(--n-dot-color-popover);")]))]);function Ne(n){return window.TouchEvent&&n instanceof window.TouchEvent}function Be(){const n=new Map,s=h=>v=>{n.set(h,v)};return mt(()=>{n.clear()}),[n,s]}const Xt=0,Yt=Object.assign(Object.assign({},ue.props),{to:ke.propTo,defaultValue:{type:[Number,Array],default:0},marks:Object,disabled:{type:Boolean,default:void 0},formatTooltip:Function,keyboard:{type:Boolean,default:!0},min:{type:Number,default:0},max:{type:Number,default:100},step:{type:[Number,String],default:1},range:Boolean,value:[Number,Array],placement:String,showTooltip:{type:Boolean,default:void 0},tooltip:{type:Boolean,default:!0},vertical:Boolean,reverse:Boolean,"onUpdate:value":[Function,Array],onUpdateValue:[Function,Array],onDragstart:[Function],onDragend:[Function]}),xe=G({name:"Slider",props:Yt,slots:Object,setup(n){const{mergedClsPrefixRef:s,namespaceRef:h,inlineThemeDisabled:v}=Ue(n),l=ue("Slider","-slider",Jt,pt,n,s),d=z(null),[S,k]=Be(),[x,p]=Be(),f=z(new Set),C=gt(n),{mergedDisabledRef:j}=C,E=R(()=>{const{step:e}=n;if(Number(e)<=0||e==="mark")return 0;const t=e.toString();let a=0;return t.includes(".")&&(a=t.length-t.indexOf(".")-1),a}),O=z(n.defaultValue),ve=bt(n,"value"),q=yt(ve,O),M=R(()=>{const{value:e}=q;return(n.range?e:[e]).map(Re)}),Q=R(()=>M.value.length>2),b=R(()=>n.placement===void 0?n.vertical?"right":"top":n.placement),$=R(()=>{const{marks:e}=n;return e?Object.keys(e).map(Number.parseFloat):null}),m=z(-1),F=z(-1),A=z(-1),N=z(!1),Z=z(!1),he=R(()=>{const{vertical:e,reverse:t}=n;return e?t?"top":"bottom":t?"right":"left"}),Fe=R(()=>{if(Q.value)return;const e=M.value,t=ee(n.range?Math.min(...e):n.min),a=ee(n.range?Math.max(...e):e[0]),{value:c}=he;return n.vertical?{[c]:`${t}%`,height:`${a-t}%`}:{[c]:`${t}%`,width:`${a-t}%`}}),je=R(()=>{const e=[],{marks:t}=n;if(t){const a=M.value.slice();a.sort((D,V)=>D-V);const{value:c}=he,{value:w}=Q,{range:T}=n,P=w?()=>!1:D=>T?D>=a[0]&&D<=a[a.length-1]:D<=a[0];for(const D of Object.keys(t)){const V=Number(D);e.push({active:P(V),key:V,label:t[D],style:{[c]:`${ee(V)}%`}})}}return e});function Ee(e,t){const a=ee(e),{value:c}=he;return{[c]:`${a}%`,zIndex:t===m.value?1:0}}function Se(e){return n.showTooltip||A.value===e||m.value===e&&N.value}function He(e){return N.value?!(m.value===e&&F.value===e):!0}function Oe(e){var t;~e&&(m.value=e,(t=S.get(e))===null||t===void 0||t.focus())}function Le(){x.forEach((e,t)=>{Se(t)&&e.syncPosition()})}function ze(e){const{"onUpdate:value":t,onUpdateValue:a}=n,{nTriggerFormInput:c,nTriggerFormChange:w}=C;a&&le(a,e),t&&le(t,e),O.value=e,c(),w()}function Ce(e){const{range:t}=n;if(t){if(Array.isArray(e)){const{value:a}=M;e.join()!==a.join()&&ze(e)}}else Array.isArray(e)||M.value[0]!==e&&ze(e)}function fe(e,t){if(n.range){const a=M.value.slice();a.splice(t,1,e),Ce(a)}else Ce(e)}function me(e,t,a){const c=a!==void 0;a||(a=e-t>0?1:-1);const w=$.value||[],{step:T}=n;if(T==="mark"){const V=te(e,w.concat(t),c?a:void 0);return V?V.value:t}if(T<=0)return t;const{value:P}=E;let D;if(c){const V=Number((t/T).toFixed(P)),H=Math.floor(V),pe=V>H?H:H-1,ge=V<H?H:H+1;D=te(t,[Number((pe*T).toFixed(P)),Number((ge*T).toFixed(P)),...w],a)}else{const V=We(e);D=te(e,[...w,V])}return D?Re(D.value):t}function Re(e){return Math.min(n.max,Math.max(n.min,e))}function ee(e){const{max:t,min:a}=n;return(e-a)/(t-a)*100}function Ke(e){const{max:t,min:a}=n;return a+(t-a)*e}function We(e){const{step:t,min:a}=n;if(Number(t)<=0||t==="mark")return e;const c=Math.round((e-a)/t)*t+a;return Number(c.toFixed(E.value))}function te(e,t=$.value,a){if(!(t!=null&&t.length))return null;let c=null,w=-1;for(;++w<t.length;){const T=t[w]-e,P=Math.abs(T);(a===void 0||T*a>0)&&(c===null||P<c.distance)&&(c={index:w,distance:P,value:t[w]})}return c}function Te(e){const t=d.value;if(!t)return;const a=Ne(e)?e.touches[0]:e,c=t.getBoundingClientRect();let w;return n.vertical?w=(c.bottom-a.clientY)/c.height:w=(a.clientX-c.left)/c.width,n.reverse&&(w=1-w),Ke(w)}function Je(e){if(j.value||!n.keyboard)return;const{vertical:t,reverse:a}=n;switch(e.key){case"ArrowUp":e.preventDefault(),ne(t&&a?-1:1);break;case"ArrowRight":e.preventDefault(),ne(!t&&a?-1:1);break;case"ArrowDown":e.preventDefault(),ne(t&&a?1:-1);break;case"ArrowLeft":e.preventDefault(),ne(!t&&a?1:-1);break}}function ne(e){const t=m.value;if(t===-1)return;const{step:a}=n,c=M.value[t],w=Number(a)<=0||a==="mark"?c:c+a*e;fe(me(w,c,e>0?1:-1),t)}function Xe(e){var t,a;if(j.value||!Ne(e)&&e.button!==Xt)return;const c=Te(e);if(c===void 0)return;const w=M.value.slice(),T=n.range?(a=(t=te(c,w))===null||t===void 0?void 0:t.index)!==null&&a!==void 0?a:-1:0;T!==-1&&(e.preventDefault(),Oe(T),Ye(),fe(me(c,M.value[T]),T))}function Ye(){N.value||(N.value=!0,n.onDragstart&&le(n.onDragstart),re("touchend",document,oe),re("mouseup",document,oe),re("touchmove",document,se),re("mousemove",document,se))}function ae(){N.value&&(N.value=!1,n.onDragend&&le(n.onDragend),de("touchend",document,oe),de("mouseup",document,oe),de("touchmove",document,se),de("mousemove",document,se))}function se(e){const{value:t}=m;if(!N.value||t===-1){ae();return}const a=Te(e);a!==void 0&&fe(me(a,M.value[t]),t)}function oe(){ae()}function Ge(e){m.value=e,j.value||(A.value=e)}function qe(e){m.value===e&&(m.value=-1,ae()),A.value===e&&(A.value=-1)}function Qe(e){A.value=e}function Ze(e){A.value===e&&(A.value=-1)}Me(m,(e,t)=>void be(()=>F.value=t)),Me(q,()=>{if(n.marks){if(Z.value)return;Z.value=!0,be(()=>{Z.value=!1})}be(Le)}),xt(()=>{ae()});const Ve=R(()=>{const{self:{markFontSize:e,railColor:t,railColorHover:a,fillColor:c,fillColorHover:w,handleColor:T,opacityDisabled:P,dotColor:D,dotColorModal:V,handleBoxShadow:H,handleBoxShadowHover:pe,handleBoxShadowActive:ge,handleBoxShadowFocus:et,dotBorder:tt,dotBoxShadow:nt,railHeight:at,railWidthVertical:st,handleSize:ot,dotHeight:it,dotWidth:lt,dotBorderRadius:rt,fontSize:dt,dotBorderActive:ct,dotColorPopover:ut},common:{cubicBezierEaseInOut:vt}}=l.value;return{"--n-bezier":vt,"--n-dot-border":tt,"--n-dot-border-active":ct,"--n-dot-border-radius":rt,"--n-dot-box-shadow":nt,"--n-dot-color":D,"--n-dot-color-modal":V,"--n-dot-color-popover":ut,"--n-dot-height":it,"--n-dot-width":lt,"--n-fill-color":c,"--n-fill-color-hover":w,"--n-font-size":dt,"--n-handle-box-shadow":H,"--n-handle-box-shadow-active":ge,"--n-handle-box-shadow-focus":et,"--n-handle-box-shadow-hover":pe,"--n-handle-color":T,"--n-handle-size":ot,"--n-opacity-disabled":P,"--n-rail-color":t,"--n-rail-color-hover":a,"--n-rail-height":at,"--n-rail-width-vertical":st,"--n-mark-font-size":e}}),W=v?we("slider",void 0,Ve,n):void 0,De=R(()=>{const{self:{fontSize:e,indicatorColor:t,indicatorBoxShadow:a,indicatorTextColor:c,indicatorBorderRadius:w}}=l.value;return{"--n-font-size":e,"--n-indicator-border-radius":w,"--n-indicator-box-shadow":a,"--n-indicator-color":t,"--n-indicator-text-color":c}}),J=v?we("slider-indicator",void 0,De,n):void 0;return{mergedClsPrefix:s,namespace:h,uncontrolledValue:O,mergedValue:q,mergedDisabled:j,mergedPlacement:b,isMounted:wt(),adjustedTo:ke(n),dotTransitionDisabled:Z,markInfos:je,isShowTooltip:Se,shouldKeepTooltipTransition:He,handleRailRef:d,setHandleRefs:k,setFollowerRefs:p,fillStyle:Fe,getHandleStyle:Ee,activeIndex:m,arrifiedValues:M,followerEnabledIndexSet:f,handleRailMouseDown:Xe,handleHandleFocus:Ge,handleHandleBlur:qe,handleHandleMouseEnter:Qe,handleHandleMouseLeave:Ze,handleRailKeyDown:Je,indicatorCssVars:v?void 0:De,indicatorThemeClass:J==null?void 0:J.themeClass,indicatorOnRender:J==null?void 0:J.onRender,cssVars:v?void 0:Ve,themeClass:W==null?void 0:W.themeClass,onRender:W==null?void 0:W.onRender}},render(){var n;const{mergedClsPrefix:s,themeClass:h,formatTooltip:v}=this;return(n=this.onRender)===null||n===void 0||n.call(this),_("div",{class:[`${s}-slider`,h,{[`${s}-slider--disabled`]:this.mergedDisabled,[`${s}-slider--active`]:this.activeIndex!==-1,[`${s}-slider--with-mark`]:this.marks,[`${s}-slider--vertical`]:this.vertical,[`${s}-slider--reverse`]:this.reverse}],style:this.cssVars,onKeydown:this.handleRailKeyDown,onMousedown:this.handleRailMouseDown,onTouchstart:this.handleRailMouseDown},_("div",{class:`${s}-slider-rail`},_("div",{class:`${s}-slider-rail__fill`,style:this.fillStyle}),this.marks?_("div",{class:[`${s}-slider-dots`,this.dotTransitionDisabled&&`${s}-slider-dots--transition-disabled`]},this.markInfos.map(l=>_("div",{key:l.key,class:[`${s}-slider-dot`,{[`${s}-slider-dot--active`]:l.active}],style:l.style}))):null,_("div",{ref:"handleRailRef",class:`${s}-slider-handles`},this.arrifiedValues.map((l,d)=>{const S=this.isShowTooltip(d);return _(kt,null,{default:()=>[_($t,null,{default:()=>_("div",{ref:this.setHandleRefs(d),class:`${s}-slider-handle-wrapper`,tabindex:this.mergedDisabled?-1:0,role:"slider","aria-valuenow":l,"aria-valuemin":this.min,"aria-valuemax":this.max,"aria-orientation":this.vertical?"vertical":"horizontal","aria-disabled":this.disabled,style:this.getHandleStyle(l,d),onFocus:()=>{this.handleHandleFocus(d)},onBlur:()=>{this.handleHandleBlur(d)},onMouseenter:()=>{this.handleHandleMouseEnter(d)},onMouseleave:()=>{this.handleHandleMouseLeave(d)}},_t(this.$slots.thumb,()=>[_("div",{class:`${s}-slider-handle`})]))}),this.tooltip&&_(St,{ref:this.setFollowerRefs(d),show:S,to:this.adjustedTo,enabled:this.showTooltip&&!this.range||this.followerEnabledIndexSet.has(d),teleportDisabled:this.adjustedTo===ke.tdkey,placement:this.mergedPlacement,containerClass:this.namespace},{default:()=>_(Ae,{name:"fade-in-scale-up-transition",appear:this.isMounted,css:this.shouldKeepTooltipTransition(d),onEnter:()=>{this.followerEnabledIndexSet.add(d)},onAfterLeave:()=>{this.followerEnabledIndexSet.delete(d)}},{default:()=>{var k;return S?((k=this.indicatorOnRender)===null||k===void 0||k.call(this),_("div",{class:[`${s}-slider-handle-indicator`,this.indicatorThemeClass,`${s}-slider-handle-indicator--${this.mergedPlacement}`],style:this.indicatorCssVars},typeof v=="function"?v(l):l)):null}})})]})})),this.marks?_("div",{class:`${s}-slider-marks`},this.markInfos.map(l=>_("div",{key:l.key,class:`${s}-slider-mark`,style:l.style},typeof l.label=="function"?l.label():l.label))):null))}}),Gt=L([L("@keyframes spin-rotate",`
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
 `,[I("rotate",`
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
 `,[I("spinning",`
 user-select: none;
 -webkit-user-select: none;
 pointer-events: none;
 opacity: var(--n-opacity-spinning);
 `)])]),qt={small:20,medium:18,large:16},Qt=Object.assign(Object.assign({},ue.props),{contentClass:String,contentStyle:[Object,String],description:String,stroke:String,size:{type:[String,Number],default:"medium"},show:{type:Boolean,default:!0},strokeWidth:Number,rotate:{type:Boolean,default:!0},spinning:{type:Boolean,validator:()=>!0,default:void 0},delay:Number}),Zt=G({name:"Spin",props:Qt,slots:Object,setup(n){const{mergedClsPrefixRef:s,inlineThemeDisabled:h}=Ue(n),v=ue("Spin","-spin",Gt,Ct,n,s),l=R(()=>{const{size:x}=n,{common:{cubicBezierEaseInOut:p},self:f}=v.value,{opacitySpinning:C,color:j,textColor:E}=f,O=typeof x=="number"?Rt(x):f[Tt("size",x)];return{"--n-bezier":p,"--n-opacity-spinning":C,"--n-size":O,"--n-color":j,"--n-text-color":E}}),d=h?we("spin",R(()=>{const{size:x}=n;return typeof x=="number"?String(x):x[0]}),l,n):void 0,S=Vt(n,["spinning","show"]),k=z(!1);return Dt(x=>{let p;if(S.value){const{delay:f}=n;if(f){p=window.setTimeout(()=>{k.value=!0},f),x(()=>{clearTimeout(p)});return}}k.value=S.value}),{mergedClsPrefix:s,active:k,mergedStrokeWidth:R(()=>{const{strokeWidth:x}=n;if(x!==void 0)return x;const{size:p}=n;return qt[typeof p=="number"?"medium":p]}),cssVars:h?void 0:l,themeClass:d==null?void 0:d.themeClass,onRender:d==null?void 0:d.onRender}},render(){var n,s;const{$slots:h,mergedClsPrefix:v,description:l}=this,d=h.icon&&this.rotate,S=(l||h.description)&&_("div",{class:`${v}-spin-description`},l||((n=h.description)===null||n===void 0?void 0:n.call(h))),k=h.icon?_("div",{class:[`${v}-spin-body`,this.themeClass]},_("div",{class:[`${v}-spin`,d&&`${v}-spin--rotate`],style:h.default?"":this.cssVars},h.icon()),S):_("div",{class:[`${v}-spin-body`,this.themeClass]},_(It,{clsPrefix:v,style:h.default?"":this.cssVars,stroke:this.stroke,"stroke-width":this.mergedStrokeWidth,class:`${v}-spin`}),S);return(s=this.onRender)===null||s===void 0||s.call(this),h.default?_("div",{class:[`${v}-spin-container`,this.themeClass],style:this.cssVars},_("div",{class:[`${v}-spin-content`,this.active&&`${v}-spin-content--spinning`,this.contentClass],style:this.contentStyle},h),_(Ae,{name:"fade-in-transition"},{default:()=>this.active?k:null})):k}});function en(){const n=new Date,s=n.getDate(),h=n.getMonth()+1;return`${n.getFullYear()}-${h}-${s}`}const tn={class:"p-4 space-y-5 min-h-[200px]"},nn={class:"space-y-6"},an={class:"flex items-center space-x-4"},sn={class:"flex-shrink-0 w-[100px]"},on={class:"flex-1"},ln={class:"flex items-center space-x-4"},rn={class:"flex-shrink-0 w-[100px]"},dn={class:"w-[200px]"},cn={class:"flex items-center space-x-4"},un={class:"flex-shrink-0 w-[100px]"},vn={class:"flex-1"},hn={class:"flex-shrink-0 w-[100px]"},fn={class:"flex flex-wrap items-center gap-4"},mn={class:"flex items-center space-x-4"},pn={class:"flex-shrink-0 w-[100px]"},gn={class:"flex flex-wrap items-center gap-4"},bn={class:"flex items-center space-x-4"},yn={class:"flex-shrink-0 w-[100px]"},xn={class:"flex flex-wrap items-center gap-4"},wn={class:"flex items-center space-x-4"},kn={class:"flex-shrink-0 w-[100px]"},$n=G({__name:"General",setup(n){const s=Mt(),h=Nt(),{isMobile:v}=Bt(),l=Pe(),d=R(()=>s.theme),S=R(()=>h.userInfo),k=z(S.value.avatar??""),x=z(S.value.name??""),p=z(S.value.description??""),f=R({get(){return s.language},set(b){s.setLanguage(b)}}),C=[{label:"Auto",key:"auto",icon:"ri:contrast-line"},{label:"Light",key:"light",icon:"ri:sun-foggy-line"},{label:"Dark",key:"dark",icon:"ri:moon-foggy-line"}],j=[{label:"English",key:"en-US",value:"en-US"},{label:"简体中文",key:"zh-CN",value:"zh-CN"},{label:"繁體中文",key:"zh-TW",value:"zh-TW"},{label:"日本語",key:"ja-JP",value:"ja-JP"},{label:"한국어",key:"ko-KR",value:"ko-KR"},{label:"Русский язык",key:"ru-RU",value:"ru-RU"},{label:"Việt nam",key:"vi-VN",value:"vi-VN"}];function E(b){h.updateUserInfo(b),l.success(X("common.success"))}function O(){h.resetUserInfo(),l.success(X("common.success")),window.location.reload()}function ve(){const b=en(),$=localStorage.getItem("chatStorage")||"{}",m=JSON.stringify(JSON.parse($),null,2),F=new Blob([m],{type:"application/json"}),A=URL.createObjectURL(F),N=document.createElement("a");N.href=A,N.download=`chat-store_${b}.json`,document.body.appendChild(N),N.click(),document.body.removeChild(N)}function q(b){const $=b.target;if(!$||!$.files)return;const m=$.files[0];if(!m)return;const F=new FileReader;F.onload=()=>{try{const A=JSON.parse(F.result);localStorage.setItem("chatStorage",JSON.stringify(A)),l.success(X("common.success")),location.reload()}catch{l.error(X("common.invalidFileFormat"))}},F.readAsText(m)}function M(){localStorage.removeItem("chatStorage"),location.reload()}function Q(){const b=document.getElementById("fileInput");b&&b.click()}return(b,$)=>(Y(),$e("div",tn,[o("div",nn,[o("div",an,[o("span",sn,g(b.$t("setting.avatarLink")),1),o("div",on,[u(r(ce),{value:k.value,"onUpdate:value":$[0]||($[0]=m=>k.value=m),placeholder:""},null,8,["value"])]),u(r(U),{size:"tiny",text:"",type:"primary",onClick:$[1]||($[1]=m=>E({avatar:k.value}))},{default:y(()=>[B(g(b.$t("common.save")),1)]),_:1})]),o("div",ln,[o("span",rn,g(b.$t("setting.name")),1),o("div",dn,[u(r(ce),{value:x.value,"onUpdate:value":$[2]||($[2]=m=>x.value=m),placeholder:""},null,8,["value"])]),u(r(U),{size:"tiny",text:"",type:"primary",onClick:$[3]||($[3]=m=>E({name:x.value}))},{default:y(()=>[B(g(b.$t("common.save")),1)]),_:1})]),o("div",cn,[o("span",un,g(b.$t("setting.description")),1),o("div",vn,[u(r(ce),{value:p.value,"onUpdate:value":$[4]||($[4]=m=>p.value=m),placeholder:""},null,8,["value"])]),u(r(U),{size:"tiny",text:"",type:"primary",onClick:$[5]||($[5]=m=>E({description:p.value}))},{default:y(()=>[B(g(b.$t("common.save")),1)]),_:1})]),o("div",{class:Ut(["flex items-center space-x-4",r(v)&&"items-start"])},[o("span",hn,g(b.$t("setting.chatHistory")),1),o("div",fn,[u(r(U),{size:"small",onClick:ve},{icon:y(()=>[u(r(K),{icon:"ri:download-2-fill"})]),default:y(()=>[B(" "+g(b.$t("common.export")),1)]),_:1}),o("input",{id:"fileInput",type:"file",style:{display:"none"},onChange:q},null,32),u(r(U),{size:"small",onClick:Q},{icon:y(()=>[u(r(K),{icon:"ri:upload-2-fill"})]),default:y(()=>[B(" "+g(b.$t("common.import")),1)]),_:1}),u(r(At),{placement:"bottom",onPositiveClick:M},{trigger:y(()=>[u(r(U),{size:"small"},{icon:y(()=>[u(r(K),{icon:"ri:close-circle-line"})]),default:y(()=>[B(" "+g(b.$t("common.clear")),1)]),_:1})]),default:y(()=>[B(" "+g(b.$t("chat.clearHistoryConfirm")),1)]),_:1})])],2),o("div",mn,[o("span",pn,g(b.$t("setting.theme")),1),o("div",gn,[(Y(),$e(Pt,null,Ft(C,m=>u(r(U),{key:m.key,size:"small",type:m.key===d.value?"primary":void 0,onClick:F=>r(s).setTheme(m.key)},{icon:y(()=>[u(r(K),{icon:m.icon},null,8,["icon"])]),_:2},1032,["type","onClick"])),64))])]),o("div",bn,[o("span",yn,g(b.$t("setting.language")),1),o("div",xn,[u(r(jt),{style:{width:"140px"},value:f.value,options:j,onUpdateValue:$[6]||($[6]=m=>r(s).setLanguage(m))},null,8,["value"])])]),o("div",wn,[o("span",kn,g(b.$t("setting.resetUserInfo")),1),u(r(U),{size:"small",onClick:O},{default:y(()=>[B(g(b.$t("common.reset")),1)]),_:1})])])]))}}),_n={class:"p-4 space-y-5 min-h-[200px]"},Sn={class:"space-y-6"},zn={class:"flex items-center space-x-4"},Cn={class:"flex-shrink-0 w-[120px]"},Rn={class:"flex-1"},Tn={class:"flex items-center space-x-4"},Vn={class:"flex-shrink-0 w-[120px]"},Dn={class:"flex-1"},In={class:"flex items-center space-x-4"},Mn={class:"flex-shrink-0 w-[120px]"},Nn={class:"flex-1"},Bn={class:"flex items-center space-x-4"},Un={class:"flex-shrink-0 w-[120px]"},An={class:"flex-1"},Pn={class:"flex items-center space-x-4"},Fn=G({__name:"Advanced",setup(n){const s=Wt(),h=Pe(),v=z(s.systemMessage??""),l=z(s.max_tokens??1e3),d=z(s.temperature??.5),S=z(s.top_p??1);function k(p){s.updateSetting(p),h.success(X("common.success"))}function x(){s.resetSetting(),h.success(X("common.success")),window.location.reload()}return(p,f)=>(Y(),$e("div",_n,[o("div",Sn,[o("div",zn,[o("span",Cn,g(p.$t("setting.role")),1),o("div",Rn,[u(r(ce),{value:v.value,"onUpdate:value":f[0]||(f[0]=C=>v.value=C),type:"textarea",autosize:{minRows:1,maxRows:4}},null,8,["value"])]),u(r(U),{size:"tiny",text:"",type:"primary",onClick:f[1]||(f[1]=C=>k({systemMessage:v.value}))},{default:y(()=>[B(g(p.$t("common.save")),1)]),_:1})]),o("div",Tn,[o("span",Vn,g(p.$t("setting.max_tokens")),1),o("div",Dn,[u(r(xe),{value:l.value,"onUpdate:value":f[2]||(f[2]=C=>l.value=C),max:10240,min:100,step:1},null,8,["value"])]),o("span",null,g(l.value),1),u(r(U),{size:"tiny",text:"",type:"primary",onClick:f[3]||(f[3]=C=>k({max_tokens:l.value}))},{default:y(()=>[B(g(p.$t("common.save")),1)]),_:1})]),o("div",In,[o("span",Mn,g(p.$t("setting.temperature")),1),o("div",Nn,[u(r(xe),{value:d.value,"onUpdate:value":f[4]||(f[4]=C=>d.value=C),max:2,min:0,step:.1},null,8,["value"])]),o("span",null,g(d.value),1),u(r(U),{size:"tiny",text:"",type:"primary",onClick:f[5]||(f[5]=C=>k({temperature:d.value}))},{default:y(()=>[B(g(p.$t("common.save")),1)]),_:1})]),o("div",Bn,[o("span",Un,g(p.$t("setting.top_p")),1),o("div",An,[u(r(xe),{value:S.value,"onUpdate:value":f[6]||(f[6]=C=>S.value=C),max:1,min:0,step:.1},null,8,["value"])]),o("span",null,g(S.value),1),u(r(U),{size:"tiny",text:"",type:"primary",onClick:f[7]||(f[7]=C=>k({top_p:S.value}))},{default:y(()=>[B(g(p.$t("common.save")),1)]),_:1})]),o("div",Pn,[f[8]||(f[8]=o("span",{class:"flex-shrink-0 w-[120px]"}," ",-1)),u(r(U),{size:"small",onClick:x},{default:y(()=>[B(g(p.$t("common.reset")),1)]),_:1})])])]))}}),jn=G({__name:"About",setup(n){const s=z(!1),h=z();async function v(){try{s.value=!0;const l={};h.value=l}finally{s.value=!1}}return Et(()=>{v()}),(l,d)=>(Y(),_e(r(Zt),{show:s.value},{default:y(()=>d[0]||(d[0]=[o("div",{class:"p-4 space-y-4"},[o("h2",{class:"text-xl font-bold"}," AIMatrices(DeepSeek) "),o("div",{class:"p-2 space-y-2 rounded-md bg-neutral-100 dark:bg-neutral-700"})],-1)])),_:1},8,["show"]))}}),En={class:"ml-2"},Hn={class:"min-h-[100px]"},On={class:"ml-2"},Ln={class:"min-h-[100px]"},Kn={class:"ml-2"},Xn=G({__name:"index",props:{visible:{type:Boolean}},emits:["update:visible"],setup(n,{emit:s}){const h=n,v=s,l=Ht(),d=R(()=>!!l.isChatGPTAPI),S=z("General"),k=R({get(){return h.visible},set(x){v("update:visible",x)}});return(x,p)=>(Y(),_e(r(Kt),{show:k.value,"onUpdate:show":p[1]||(p[1]=f=>k.value=f),"auto-focus":!1,preset:"card",style:{width:"95%","max-width":"640px"}},{default:y(()=>[o("div",null,[u(r(Ot),{value:S.value,"onUpdate:value":p[0]||(p[0]=f=>S.value=f),type:"line",animated:""},{default:y(()=>[u(r(ye),{name:"General",tab:"General"},{tab:y(()=>[u(r(K),{class:"text-lg",icon:"ri:file-user-line"}),o("span",En,g(x.$t("setting.general")),1)]),default:y(()=>[o("div",Hn,[u($n)])]),_:1}),d.value?(Y(),_e(r(ye),{key:0,name:"Advanced",tab:"Advanced"},{tab:y(()=>[u(r(K),{class:"text-lg",icon:"ri:equalizer-line"}),o("span",On,g(x.$t("setting.advanced")),1)]),default:y(()=>[o("div",Ln,[u(Fn)])]),_:1})):Lt("",!0),u(r(ye),{name:"About",tab:"About"},{tab:y(()=>[u(r(K),{class:"text-lg",icon:"ri:list-settings-line"}),o("span",Kn,g(x.$t("setting.about")),1)]),default:y(()=>[u(jn)]),_:1})]),_:1},8,["value"])])]),_:1},8,["show"]))}});export{Xn as default};
