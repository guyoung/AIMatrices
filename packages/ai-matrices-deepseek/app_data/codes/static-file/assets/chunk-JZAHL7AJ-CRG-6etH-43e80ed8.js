import{u as xe,b as $e}from"./chunk-EICJXIV7-DR9XTAAH-1c5dda2c.js";import{d as i,e as M,c as D,U as ve,a as De,G as Ce,Y as Le,m as Ie,X as Ae,$ as k,W as we,r as Re,t as Ne,aA as Oe}from"./index-8de624c0.js";var At=function(){var t=i(function(L,n,a,y){for(a=a||{},y=L.length;y--;a[L[y]]=n);return a},"o"),e=[1,2],l=[1,3],o=[1,4],r=[2,4],h=[1,9],p=[1,11],f=[1,16],u=[1,17],S=[1,18],_=[1,19],I=[1,32],V=[1,20],Y=[1,21],A=[1,22],d=[1,23],C=[1,24],w=[1,26],P=[1,27],G=[1,28],N=[1,29],O=[1,30],it=[1,31],rt=[1,34],nt=[1,35],at=[1,36],ot=[1,37],K=[1,33],g=[1,4,5,16,17,19,21,22,24,25,26,27,28,29,33,35,37,38,42,45,48,49,50,51,54],lt=[1,4,5,14,15,16,17,19,21,22,24,25,26,27,28,29,33,35,37,38,42,45,48,49,50,51,54],Gt=[4,5,16,17,19,21,22,24,25,26,27,28,29,33,35,37,38,42,45,48,49,50,51,54],Et={trace:i(function(){},"trace"),yy:{},symbols_:{error:2,start:3,SPACE:4,NL:5,SD:6,document:7,line:8,statement:9,classDefStatement:10,styleStatement:11,cssClassStatement:12,idStatement:13,DESCR:14,"-->":15,HIDE_EMPTY:16,scale:17,WIDTH:18,COMPOSIT_STATE:19,STRUCT_START:20,STRUCT_STOP:21,STATE_DESCR:22,AS:23,ID:24,FORK:25,JOIN:26,CHOICE:27,CONCURRENT:28,note:29,notePosition:30,NOTE_TEXT:31,direction:32,acc_title:33,acc_title_value:34,acc_descr:35,acc_descr_value:36,acc_descr_multiline_value:37,classDef:38,CLASSDEF_ID:39,CLASSDEF_STYLEOPTS:40,DEFAULT:41,style:42,STYLE_IDS:43,STYLEDEF_STYLEOPTS:44,class:45,CLASSENTITY_IDS:46,STYLECLASS:47,direction_tb:48,direction_bt:49,direction_rl:50,direction_lr:51,eol:52,";":53,EDGE_STATE:54,STYLE_SEPARATOR:55,left_of:56,right_of:57,$accept:0,$end:1},terminals_:{2:"error",4:"SPACE",5:"NL",6:"SD",14:"DESCR",15:"-->",16:"HIDE_EMPTY",17:"scale",18:"WIDTH",19:"COMPOSIT_STATE",20:"STRUCT_START",21:"STRUCT_STOP",22:"STATE_DESCR",23:"AS",24:"ID",25:"FORK",26:"JOIN",27:"CHOICE",28:"CONCURRENT",29:"note",31:"NOTE_TEXT",33:"acc_title",34:"acc_title_value",35:"acc_descr",36:"acc_descr_value",37:"acc_descr_multiline_value",38:"classDef",39:"CLASSDEF_ID",40:"CLASSDEF_STYLEOPTS",41:"DEFAULT",42:"style",43:"STYLE_IDS",44:"STYLEDEF_STYLEOPTS",45:"class",46:"CLASSENTITY_IDS",47:"STYLECLASS",48:"direction_tb",49:"direction_bt",50:"direction_rl",51:"direction_lr",53:";",54:"EDGE_STATE",55:"STYLE_SEPARATOR",56:"left_of",57:"right_of"},productions_:[0,[3,2],[3,2],[3,2],[7,0],[7,2],[8,2],[8,1],[8,1],[9,1],[9,1],[9,1],[9,1],[9,2],[9,3],[9,4],[9,1],[9,2],[9,1],[9,4],[9,3],[9,6],[9,1],[9,1],[9,1],[9,1],[9,4],[9,4],[9,1],[9,2],[9,2],[9,1],[10,3],[10,3],[11,3],[12,3],[32,1],[32,1],[32,1],[32,1],[52,1],[52,1],[13,1],[13,1],[13,3],[13,3],[30,1],[30,1]],performAction:i(function(L,n,a,y,m,s,T){var c=s.length-1;switch(m){case 3:return y.setRootDoc(s[c]),s[c];case 4:this.$=[];break;case 5:s[c]!="nl"&&(s[c-1].push(s[c]),this.$=s[c-1]);break;case 6:case 7:this.$=s[c];break;case 8:this.$="nl";break;case 12:this.$=s[c];break;case 13:const q=s[c-1];q.description=y.trimColon(s[c]),this.$=q;break;case 14:this.$={stmt:"relation",state1:s[c-2],state2:s[c]};break;case 15:const xt=y.trimColon(s[c]);this.$={stmt:"relation",state1:s[c-3],state2:s[c-1],description:xt};break;case 19:this.$={stmt:"state",id:s[c-3],type:"default",description:"",doc:s[c-1]};break;case 20:var z=s[c],W=s[c-2].trim();if(s[c].match(":")){var ht=s[c].split(":");z=ht[0],W=[W,ht[1]]}this.$={stmt:"state",id:z,type:"default",description:W};break;case 21:this.$={stmt:"state",id:s[c-3],type:"default",description:s[c-5],doc:s[c-1]};break;case 22:this.$={stmt:"state",id:s[c],type:"fork"};break;case 23:this.$={stmt:"state",id:s[c],type:"join"};break;case 24:this.$={stmt:"state",id:s[c],type:"choice"};break;case 25:this.$={stmt:"state",id:y.getDividerId(),type:"divider"};break;case 26:this.$={stmt:"state",id:s[c-1].trim(),note:{position:s[c-2].trim(),text:s[c].trim()}};break;case 29:this.$=s[c].trim(),y.setAccTitle(this.$);break;case 30:case 31:this.$=s[c].trim(),y.setAccDescription(this.$);break;case 32:case 33:this.$={stmt:"classDef",id:s[c-1].trim(),classes:s[c].trim()};break;case 34:this.$={stmt:"style",id:s[c-1].trim(),styleClass:s[c].trim()};break;case 35:this.$={stmt:"applyClass",id:s[c-1].trim(),styleClass:s[c].trim()};break;case 36:y.setDirection("TB"),this.$={stmt:"dir",value:"TB"};break;case 37:y.setDirection("BT"),this.$={stmt:"dir",value:"BT"};break;case 38:y.setDirection("RL"),this.$={stmt:"dir",value:"RL"};break;case 39:y.setDirection("LR"),this.$={stmt:"dir",value:"LR"};break;case 42:case 43:this.$={stmt:"state",id:s[c].trim(),type:"default",description:""};break;case 44:this.$={stmt:"state",id:s[c-2].trim(),classes:[s[c].trim()],type:"default",description:""};break;case 45:this.$={stmt:"state",id:s[c-2].trim(),classes:[s[c].trim()],type:"default",description:""};break}},"anonymous"),table:[{3:1,4:e,5:l,6:o},{1:[3]},{3:5,4:e,5:l,6:o},{3:6,4:e,5:l,6:o},t([1,4,5,16,17,19,22,24,25,26,27,28,29,33,35,37,38,42,45,48,49,50,51,54],r,{7:7}),{1:[2,1]},{1:[2,2]},{1:[2,3],4:h,5:p,8:8,9:10,10:12,11:13,12:14,13:15,16:f,17:u,19:S,22:_,24:I,25:V,26:Y,27:A,28:d,29:C,32:25,33:w,35:P,37:G,38:N,42:O,45:it,48:rt,49:nt,50:at,51:ot,54:K},t(g,[2,5]),{9:38,10:12,11:13,12:14,13:15,16:f,17:u,19:S,22:_,24:I,25:V,26:Y,27:A,28:d,29:C,32:25,33:w,35:P,37:G,38:N,42:O,45:it,48:rt,49:nt,50:at,51:ot,54:K},t(g,[2,7]),t(g,[2,8]),t(g,[2,9]),t(g,[2,10]),t(g,[2,11]),t(g,[2,12],{14:[1,39],15:[1,40]}),t(g,[2,16]),{18:[1,41]},t(g,[2,18],{20:[1,42]}),{23:[1,43]},t(g,[2,22]),t(g,[2,23]),t(g,[2,24]),t(g,[2,25]),{30:44,31:[1,45],56:[1,46],57:[1,47]},t(g,[2,28]),{34:[1,48]},{36:[1,49]},t(g,[2,31]),{39:[1,50],41:[1,51]},{43:[1,52]},{46:[1,53]},t(lt,[2,42],{55:[1,54]}),t(lt,[2,43],{55:[1,55]}),t(g,[2,36]),t(g,[2,37]),t(g,[2,38]),t(g,[2,39]),t(g,[2,6]),t(g,[2,13]),{13:56,24:I,54:K},t(g,[2,17]),t(Gt,r,{7:57}),{24:[1,58]},{24:[1,59]},{23:[1,60]},{24:[2,46]},{24:[2,47]},t(g,[2,29]),t(g,[2,30]),{40:[1,61]},{40:[1,62]},{44:[1,63]},{47:[1,64]},{24:[1,65]},{24:[1,66]},t(g,[2,14],{14:[1,67]}),{4:h,5:p,8:8,9:10,10:12,11:13,12:14,13:15,16:f,17:u,19:S,21:[1,68],22:_,24:I,25:V,26:Y,27:A,28:d,29:C,32:25,33:w,35:P,37:G,38:N,42:O,45:it,48:rt,49:nt,50:at,51:ot,54:K},t(g,[2,20],{20:[1,69]}),{31:[1,70]},{24:[1,71]},t(g,[2,32]),t(g,[2,33]),t(g,[2,34]),t(g,[2,35]),t(lt,[2,44]),t(lt,[2,45]),t(g,[2,15]),t(g,[2,19]),t(Gt,r,{7:72}),t(g,[2,26]),t(g,[2,27]),{4:h,5:p,8:8,9:10,10:12,11:13,12:14,13:15,16:f,17:u,19:S,21:[1,73],22:_,24:I,25:V,26:Y,27:A,28:d,29:C,32:25,33:w,35:P,37:G,38:N,42:O,45:it,48:rt,49:nt,50:at,51:ot,54:K},t(g,[2,21])],defaultActions:{5:[2,1],6:[2,2],46:[2,46],47:[2,47]},parseError:i(function(L,n){if(n.recoverable)this.trace(L);else{var a=new Error(L);throw a.hash=n,a}},"parseError"),parse:i(function(L){var n=this,a=[0],y=[],m=[null],s=[],T=this.table,c="",z=0,W=0,ht=2,q=1,xt=s.slice.call(arguments,1),b=Object.create(this.lexer),U={yy:{}};for(var $t in this.yy)Object.prototype.hasOwnProperty.call(this.yy,$t)&&(U.yy[$t]=this.yy[$t]);b.setInput(L,U.yy),U.yy.lexer=b,U.yy.parser=this,typeof b.yylloc>"u"&&(b.yylloc={});var vt=b.yylloc;s.push(vt);var Te=b.options&&b.options.ranges;typeof U.yy.parseError=="function"?this.parseError=U.yy.parseError:this.parseError=Object.getPrototypeOf(this).parseError;function Ee($){a.length=a.length-2*$,m.length=m.length-$,s.length=s.length-$}i(Ee,"popStack");function zt(){var $;return $=y.pop()||b.lex()||q,typeof $!="number"&&($ instanceof Array&&(y=$,$=y.pop()),$=n.symbols_[$]||$),$}i(zt,"lex");for(var E,j,v,Dt,J={},ut,R,Ut,dt;;){if(j=a[a.length-1],this.defaultActions[j]?v=this.defaultActions[j]:((E===null||typeof E>"u")&&(E=zt()),v=T[j]&&T[j][E]),typeof v>"u"||!v.length||!v[0]){var Ct="";dt=[];for(ut in T[j])this.terminals_[ut]&&ut>ht&&dt.push("'"+this.terminals_[ut]+"'");b.showPosition?Ct="Parse error on line "+(z+1)+`:
`+b.showPosition()+`
Expecting `+dt.join(", ")+", got '"+(this.terminals_[E]||E)+"'":Ct="Parse error on line "+(z+1)+": Unexpected "+(E==q?"end of input":"'"+(this.terminals_[E]||E)+"'"),this.parseError(Ct,{text:b.match,token:this.terminals_[E]||E,line:b.yylineno,loc:vt,expected:dt})}if(v[0]instanceof Array&&v.length>1)throw new Error("Parse Error: multiple actions possible at state: "+j+", token: "+E);switch(v[0]){case 1:a.push(E),m.push(b.yytext),s.push(b.yylloc),a.push(v[1]),E=null,W=b.yyleng,c=b.yytext,z=b.yylineno,vt=b.yylloc;break;case 2:if(R=this.productions_[v[1]][1],J.$=m[m.length-R],J._$={first_line:s[s.length-(R||1)].first_line,last_line:s[s.length-1].last_line,first_column:s[s.length-(R||1)].first_column,last_column:s[s.length-1].last_column},Te&&(J._$.range=[s[s.length-(R||1)].range[0],s[s.length-1].range[1]]),Dt=this.performAction.apply(J,[c,W,z,U.yy,v[1],m,s].concat(xt)),typeof Dt<"u")return Dt;R&&(a=a.slice(0,-1*R*2),m=m.slice(0,-1*R),s=s.slice(0,-1*R)),a.push(this.productions_[v[1]][0]),m.push(J.$),s.push(J._$),Ut=T[a[a.length-2]][a[a.length-1]],a.push(Ut);break;case 3:return!0}}return!0},"parse")},ke=function(){var L={EOF:1,parseError:i(function(n,a){if(this.yy.parser)this.yy.parser.parseError(n,a);else throw new Error(n)},"parseError"),setInput:i(function(n,a){return this.yy=a||this.yy||{},this._input=n,this._more=this._backtrack=this.done=!1,this.yylineno=this.yyleng=0,this.yytext=this.matched=this.match="",this.conditionStack=["INITIAL"],this.yylloc={first_line:1,first_column:0,last_line:1,last_column:0},this.options.ranges&&(this.yylloc.range=[0,0]),this.offset=0,this},"setInput"),input:i(function(){var n=this._input[0];this.yytext+=n,this.yyleng++,this.offset++,this.match+=n,this.matched+=n;var a=n.match(/(?:\r\n?|\n).*/g);return a?(this.yylineno++,this.yylloc.last_line++):this.yylloc.last_column++,this.options.ranges&&this.yylloc.range[1]++,this._input=this._input.slice(1),n},"input"),unput:i(function(n){var a=n.length,y=n.split(/(?:\r\n?|\n)/g);this._input=n+this._input,this.yytext=this.yytext.substr(0,this.yytext.length-a),this.offset-=a;var m=this.match.split(/(?:\r\n?|\n)/g);this.match=this.match.substr(0,this.match.length-1),this.matched=this.matched.substr(0,this.matched.length-1),y.length-1&&(this.yylineno-=y.length-1);var s=this.yylloc.range;return this.yylloc={first_line:this.yylloc.first_line,last_line:this.yylineno+1,first_column:this.yylloc.first_column,last_column:y?(y.length===m.length?this.yylloc.first_column:0)+m[m.length-y.length].length-y[0].length:this.yylloc.first_column-a},this.options.ranges&&(this.yylloc.range=[s[0],s[0]+this.yyleng-a]),this.yyleng=this.yytext.length,this},"unput"),more:i(function(){return this._more=!0,this},"more"),reject:i(function(){if(this.options.backtrack_lexer)this._backtrack=!0;else return this.parseError("Lexical error on line "+(this.yylineno+1)+`. You can only invoke reject() in the lexer when the lexer is of the backtracking persuasion (options.backtrack_lexer = true).
`+this.showPosition(),{text:"",token:null,line:this.yylineno});return this},"reject"),less:i(function(n){this.unput(this.match.slice(n))},"less"),pastInput:i(function(){var n=this.matched.substr(0,this.matched.length-this.match.length);return(n.length>20?"...":"")+n.substr(-20).replace(/\n/g,"")},"pastInput"),upcomingInput:i(function(){var n=this.match;return n.length<20&&(n+=this._input.substr(0,20-n.length)),(n.substr(0,20)+(n.length>20?"...":"")).replace(/\n/g,"")},"upcomingInput"),showPosition:i(function(){var n=this.pastInput(),a=new Array(n.length+1).join("-");return n+this.upcomingInput()+`
`+a+"^"},"showPosition"),test_match:i(function(n,a){var y,m,s;if(this.options.backtrack_lexer&&(s={yylineno:this.yylineno,yylloc:{first_line:this.yylloc.first_line,last_line:this.last_line,first_column:this.yylloc.first_column,last_column:this.yylloc.last_column},yytext:this.yytext,match:this.match,matches:this.matches,matched:this.matched,yyleng:this.yyleng,offset:this.offset,_more:this._more,_input:this._input,yy:this.yy,conditionStack:this.conditionStack.slice(0),done:this.done},this.options.ranges&&(s.yylloc.range=this.yylloc.range.slice(0))),m=n[0].match(/(?:\r\n?|\n).*/g),m&&(this.yylineno+=m.length),this.yylloc={first_line:this.yylloc.last_line,last_line:this.yylineno+1,first_column:this.yylloc.last_column,last_column:m?m[m.length-1].length-m[m.length-1].match(/\r?\n?/)[0].length:this.yylloc.last_column+n[0].length},this.yytext+=n[0],this.match+=n[0],this.matches=n,this.yyleng=this.yytext.length,this.options.ranges&&(this.yylloc.range=[this.offset,this.offset+=this.yyleng]),this._more=!1,this._backtrack=!1,this._input=this._input.slice(n[0].length),this.matched+=n[0],y=this.performAction.call(this,this.yy,this,a,this.conditionStack[this.conditionStack.length-1]),this.done&&this._input&&(this.done=!1),y)return y;if(this._backtrack){for(var T in s)this[T]=s[T];return!1}return!1},"test_match"),next:i(function(){if(this.done)return this.EOF;this._input||(this.done=!0);var n,a,y,m;this._more||(this.yytext="",this.match="");for(var s=this._currentRules(),T=0;T<s.length;T++)if(y=this._input.match(this.rules[s[T]]),y&&(!a||y[0].length>a[0].length)){if(a=y,m=T,this.options.backtrack_lexer){if(n=this.test_match(y,s[T]),n!==!1)return n;if(this._backtrack){a=!1;continue}else return!1}else if(!this.options.flex)break}return a?(n=this.test_match(a,s[m]),n!==!1?n:!1):this._input===""?this.EOF:this.parseError("Lexical error on line "+(this.yylineno+1)+`. Unrecognized text.
`+this.showPosition(),{text:"",token:null,line:this.yylineno})},"next"),lex:i(function(){var n=this.next();return n||this.lex()},"lex"),begin:i(function(n){this.conditionStack.push(n)},"begin"),popState:i(function(){var n=this.conditionStack.length-1;return n>0?this.conditionStack.pop():this.conditionStack[0]},"popState"),_currentRules:i(function(){return this.conditionStack.length&&this.conditionStack[this.conditionStack.length-1]?this.conditions[this.conditionStack[this.conditionStack.length-1]].rules:this.conditions.INITIAL.rules},"_currentRules"),topState:i(function(n){return n=this.conditionStack.length-1-Math.abs(n||0),n>=0?this.conditionStack[n]:"INITIAL"},"topState"),pushState:i(function(n){this.begin(n)},"pushState"),stateStackSize:i(function(){return this.conditionStack.length},"stateStackSize"),options:{"case-insensitive":!0},performAction:i(function(n,a,y,m){switch(y){case 0:return 41;case 1:return 48;case 2:return 49;case 3:return 50;case 4:return 51;case 5:break;case 6:break;case 7:return 5;case 8:break;case 9:break;case 10:break;case 11:break;case 12:return this.pushState("SCALE"),17;case 13:return 18;case 14:this.popState();break;case 15:return this.begin("acc_title"),33;case 16:return this.popState(),"acc_title_value";case 17:return this.begin("acc_descr"),35;case 18:return this.popState(),"acc_descr_value";case 19:this.begin("acc_descr_multiline");break;case 20:this.popState();break;case 21:return"acc_descr_multiline_value";case 22:return this.pushState("CLASSDEF"),38;case 23:return this.popState(),this.pushState("CLASSDEFID"),"DEFAULT_CLASSDEF_ID";case 24:return this.popState(),this.pushState("CLASSDEFID"),39;case 25:return this.popState(),40;case 26:return this.pushState("CLASS"),45;case 27:return this.popState(),this.pushState("CLASS_STYLE"),46;case 28:return this.popState(),47;case 29:return this.pushState("STYLE"),42;case 30:return this.popState(),this.pushState("STYLEDEF_STYLES"),43;case 31:return this.popState(),44;case 32:return this.pushState("SCALE"),17;case 33:return 18;case 34:this.popState();break;case 35:this.pushState("STATE");break;case 36:return this.popState(),a.yytext=a.yytext.slice(0,-8).trim(),25;case 37:return this.popState(),a.yytext=a.yytext.slice(0,-8).trim(),26;case 38:return this.popState(),a.yytext=a.yytext.slice(0,-10).trim(),27;case 39:return this.popState(),a.yytext=a.yytext.slice(0,-8).trim(),25;case 40:return this.popState(),a.yytext=a.yytext.slice(0,-8).trim(),26;case 41:return this.popState(),a.yytext=a.yytext.slice(0,-10).trim(),27;case 42:return 48;case 43:return 49;case 44:return 50;case 45:return 51;case 46:this.pushState("STATE_STRING");break;case 47:return this.pushState("STATE_ID"),"AS";case 48:return this.popState(),"ID";case 49:this.popState();break;case 50:return"STATE_DESCR";case 51:return 19;case 52:this.popState();break;case 53:return this.popState(),this.pushState("struct"),20;case 54:break;case 55:return this.popState(),21;case 56:break;case 57:return this.begin("NOTE"),29;case 58:return this.popState(),this.pushState("NOTE_ID"),56;case 59:return this.popState(),this.pushState("NOTE_ID"),57;case 60:this.popState(),this.pushState("FLOATING_NOTE");break;case 61:return this.popState(),this.pushState("FLOATING_NOTE_ID"),"AS";case 62:break;case 63:return"NOTE_TEXT";case 64:return this.popState(),"ID";case 65:return this.popState(),this.pushState("NOTE_TEXT"),24;case 66:return this.popState(),a.yytext=a.yytext.substr(2).trim(),31;case 67:return this.popState(),a.yytext=a.yytext.slice(0,-8).trim(),31;case 68:return 6;case 69:return 6;case 70:return 16;case 71:return 54;case 72:return 24;case 73:return a.yytext=a.yytext.trim(),14;case 74:return 15;case 75:return 28;case 76:return 55;case 77:return 5;case 78:return"INVALID"}},"anonymous"),rules:[/^(?:default\b)/i,/^(?:.*direction\s+TB[^\n]*)/i,/^(?:.*direction\s+BT[^\n]*)/i,/^(?:.*direction\s+RL[^\n]*)/i,/^(?:.*direction\s+LR[^\n]*)/i,/^(?:%%(?!\{)[^\n]*)/i,/^(?:[^\}]%%[^\n]*)/i,/^(?:[\n]+)/i,/^(?:[\s]+)/i,/^(?:((?!\n)\s)+)/i,/^(?:#[^\n]*)/i,/^(?:%[^\n]*)/i,/^(?:scale\s+)/i,/^(?:\d+)/i,/^(?:\s+width\b)/i,/^(?:accTitle\s*:\s*)/i,/^(?:(?!\n||)*[^\n]*)/i,/^(?:accDescr\s*:\s*)/i,/^(?:(?!\n||)*[^\n]*)/i,/^(?:accDescr\s*\{\s*)/i,/^(?:[\}])/i,/^(?:[^\}]*)/i,/^(?:classDef\s+)/i,/^(?:DEFAULT\s+)/i,/^(?:\w+\s+)/i,/^(?:[^\n]*)/i,/^(?:class\s+)/i,/^(?:(\w+)+((,\s*\w+)*))/i,/^(?:[^\n]*)/i,/^(?:style\s+)/i,/^(?:[\w,]+\s+)/i,/^(?:[^\n]*)/i,/^(?:scale\s+)/i,/^(?:\d+)/i,/^(?:\s+width\b)/i,/^(?:state\s+)/i,/^(?:.*<<fork>>)/i,/^(?:.*<<join>>)/i,/^(?:.*<<choice>>)/i,/^(?:.*\[\[fork\]\])/i,/^(?:.*\[\[join\]\])/i,/^(?:.*\[\[choice\]\])/i,/^(?:.*direction\s+TB[^\n]*)/i,/^(?:.*direction\s+BT[^\n]*)/i,/^(?:.*direction\s+RL[^\n]*)/i,/^(?:.*direction\s+LR[^\n]*)/i,/^(?:["])/i,/^(?:\s*as\s+)/i,/^(?:[^\n\{]*)/i,/^(?:["])/i,/^(?:[^"]*)/i,/^(?:[^\n\s\{]+)/i,/^(?:\n)/i,/^(?:\{)/i,/^(?:%%(?!\{)[^\n]*)/i,/^(?:\})/i,/^(?:[\n])/i,/^(?:note\s+)/i,/^(?:left of\b)/i,/^(?:right of\b)/i,/^(?:")/i,/^(?:\s*as\s*)/i,/^(?:["])/i,/^(?:[^"]*)/i,/^(?:[^\n]*)/i,/^(?:\s*[^:\n\s\-]+)/i,/^(?:\s*:[^:\n;]+)/i,/^(?:[\s\S]*?end note\b)/i,/^(?:stateDiagram\s+)/i,/^(?:stateDiagram-v2\s+)/i,/^(?:hide empty description\b)/i,/^(?:\[\*\])/i,/^(?:[^:\n\s\-\{]+)/i,/^(?:\s*:[^:\n;]+)/i,/^(?:-->)/i,/^(?:--)/i,/^(?::::)/i,/^(?:$)/i,/^(?:.)/i],conditions:{LINE:{rules:[9,10],inclusive:!1},struct:{rules:[9,10,22,26,29,35,42,43,44,45,54,55,56,57,71,72,73,74,75],inclusive:!1},FLOATING_NOTE_ID:{rules:[64],inclusive:!1},FLOATING_NOTE:{rules:[61,62,63],inclusive:!1},NOTE_TEXT:{rules:[66,67],inclusive:!1},NOTE_ID:{rules:[65],inclusive:!1},NOTE:{rules:[58,59,60],inclusive:!1},STYLEDEF_STYLEOPTS:{rules:[],inclusive:!1},STYLEDEF_STYLES:{rules:[31],inclusive:!1},STYLE_IDS:{rules:[],inclusive:!1},STYLE:{rules:[30],inclusive:!1},CLASS_STYLE:{rules:[28],inclusive:!1},CLASS:{rules:[27],inclusive:!1},CLASSDEFID:{rules:[25],inclusive:!1},CLASSDEF:{rules:[23,24],inclusive:!1},acc_descr_multiline:{rules:[20,21],inclusive:!1},acc_descr:{rules:[18],inclusive:!1},acc_title:{rules:[16],inclusive:!1},SCALE:{rules:[13,14,33,34],inclusive:!1},ALIAS:{rules:[],inclusive:!1},STATE_ID:{rules:[48],inclusive:!1},STATE_STRING:{rules:[49,50],inclusive:!1},FORK_STATE:{rules:[],inclusive:!1},STATE:{rules:[9,10,36,37,38,39,40,41,46,47,51,52,53],inclusive:!1},ID:{rules:[9,10],inclusive:!1},INITIAL:{rules:[0,1,2,3,4,5,6,7,8,10,11,12,15,17,19,22,26,29,32,35,53,57,68,69,70,71,72,73,74,76,77,78],inclusive:!0}}};return L}();Et.lexer=ke;function ct(){this.yy={}}return i(ct,"Parser"),ct.prototype=Et,Et.Parser=ct,new ct}();At.parser=At;var Ds=At,Be="LR",Kt="TB",St="state",Rt="relation",Fe="classDef",Ye="style",Pe="applyClass",et="default",qt="divider",Qt="fill:none",Zt="fill: #333",te="c",ee="text",se="normal",Lt="rect",It="rectWithTitle",Ge="stateStart",ze="stateEnd",jt="divider",Mt="roundedWithTitle",Ue="note",je="noteGroup",st="statediagram",Me="state",He=`${st}-${Me}`,ie="transition",Ve="note",We="note-edge",Je=`${ie} ${We}`,Xe=`${st}-${Ve}`,Ke="cluster",qe=`${st}-${Ke}`,Qe="cluster-alt",Ze=`${st}-${Qe}`,re="parent",ne="note",ts="state",Nt="----",es=`${Nt}${ne}`,Ht=`${Nt}${re}`,ae=i((t,e=Kt)=>{if(!t.doc)return e;let l=e;for(const o of t.doc)o.stmt==="dir"&&(l=o.value);return l},"getDir"),ss=i(function(t,e){return e.db.extract(e.db.getRootDocV2()),e.db.getClasses()},"getClasses"),is=i(async function(t,e,l,o){k.info("REF0:"),k.info("Drawing state diagram (v2)",e);const{securityLevel:r,state:h,layout:p}=D();o.db.extract(o.db.getRootDocV2());const f=o.db.getData(),u=xe(e,r);f.type=o.type,f.layoutAlgorithm=p,f.nodeSpacing=(h==null?void 0:h.nodeSpacing)||50,f.rankSpacing=(h==null?void 0:h.rankSpacing)||50,f.markers=["barb"],f.diagramId=e,await we(f,u);const S=8;Re.insertTitle(u,"statediagramTitleText",(h==null?void 0:h.titleTopMargin)??25,o.db.getDiagramTitle()),$e(u,S,st,(h==null?void 0:h.useMaxWidth)??!0)},"draw"),Cs={getClasses:ss,draw:is,getDir:ae},yt=new Map,B=0;function gt(t="",e=0,l="",o=Nt){const r=l!==null&&l.length>0?`${o}${l}`:"";return`${ts}-${t}${r}-${e}`}i(gt,"stateDomId");var rs=i((t,e,l,o,r,h,p,f)=>{k.trace("items",e),e.forEach(u=>{switch(u.stmt){case St:Z(t,u,l,o,r,h,p,f);break;case et:Z(t,u,l,o,r,h,p,f);break;case Rt:{Z(t,u.state1,l,o,r,h,p,f),Z(t,u.state2,l,o,r,h,p,f);const S={id:"edge"+B,start:u.state1.id,end:u.state2.id,arrowhead:"normal",arrowTypeEnd:"arrow_barb",style:Qt,labelStyle:"",label:M.sanitizeText(u.description,D()),arrowheadStyle:Zt,labelpos:te,labelType:ee,thickness:se,classes:ie,look:p};r.push(S),B++}break}})},"setupDoc"),Vt=i((t,e=Kt)=>{let l=e;if(t.doc)for(const o of t.doc)o.stmt==="dir"&&(l=o.value);return l},"getDir");function Q(t,e,l){if(!e.id||e.id==="</join></fork>"||e.id==="</choice>")return;e.cssClasses&&(Array.isArray(e.cssCompiledStyles)||(e.cssCompiledStyles=[]),e.cssClasses.split(" ").forEach(r=>{if(l.get(r)){const h=l.get(r);e.cssCompiledStyles=[...e.cssCompiledStyles,...h.styles]}}));const o=t.find(r=>r.id===e.id);o?Object.assign(o,e):t.push(e)}i(Q,"insertOrUpdateNode");function oe(t){var e;return((e=t==null?void 0:t.classes)==null?void 0:e.join(" "))??""}i(oe,"getClassesFromDbInfo");function le(t){return(t==null?void 0:t.styles)??[]}i(le,"getStylesFromDbInfo");var Z=i((t,e,l,o,r,h,p,f)=>{var u,S;const _=e.id,I=l.get(_),V=oe(I),Y=le(I);if(k.info("dataFetcher parsedItem",e,I,Y),_!=="root"){let A=Lt;e.start===!0?A=Ge:e.start===!1&&(A=ze),e.type!==et&&(A=e.type),yt.get(_)||yt.set(_,{id:_,shape:A,description:M.sanitizeText(_,D()),cssClasses:`${V} ${He}`,cssStyles:Y});const d=yt.get(_);e.description&&(Array.isArray(d.description)?(d.shape=It,d.description.push(e.description)):((u=d.description)==null?void 0:u.length)>0?(d.shape=It,d.description===_?d.description=[e.description]:d.description=[d.description,e.description]):(d.shape=Lt,d.description=e.description),d.description=M.sanitizeTextOrArray(d.description,D())),((S=d.description)==null?void 0:S.length)===1&&d.shape===It&&(d.type==="group"?d.shape=Mt:d.shape=Lt),!d.type&&e.doc&&(k.info("Setting cluster for XCX",_,Vt(e)),d.type="group",d.isGroup=!0,d.dir=Vt(e),d.shape=e.type===qt?jt:Mt,d.cssClasses=`${d.cssClasses} ${qe} ${h?Ze:""}`);const C={labelStyle:"",shape:d.shape,label:d.description,cssClasses:d.cssClasses,cssCompiledStyles:[],cssStyles:d.cssStyles,id:_,dir:d.dir,domId:gt(_,B),type:d.type,isGroup:d.type==="group",padding:8,rx:10,ry:10,look:p};if(C.shape===jt&&(C.label=""),t&&t.id!=="root"&&(k.trace("Setting node ",_," to be child of its parent ",t.id),C.parentId=t.id),C.centerLabel=!0,e.note){const w={labelStyle:"",shape:Ue,label:e.note.text,cssClasses:Xe,cssStyles:[],cssCompilesStyles:[],id:_+es+"-"+B,domId:gt(_,B,ne),type:d.type,isGroup:d.type==="group",padding:D().flowchart.padding,look:p,position:e.note.position},P=_+Ht,G={labelStyle:"",shape:je,label:e.note.text,cssClasses:d.cssClasses,cssStyles:[],id:_+Ht,domId:gt(_,B,re),type:"group",isGroup:!0,padding:16,look:p,position:e.note.position};B++,G.id=P,w.parentId=P,Q(o,G,f),Q(o,w,f),Q(o,C,f);let N=_,O=w.id;e.note.position==="left of"&&(N=w.id,O=_),r.push({id:N+"-"+O,start:N,end:O,arrowhead:"none",arrowTypeEnd:"",style:Qt,labelStyle:"",classes:Je,arrowheadStyle:Zt,labelpos:te,labelType:ee,thickness:se,look:p})}else Q(o,C,f)}e.doc&&(k.trace("Adding nodes children "),rs(e,e.doc,l,o,r,!h,p,f))},"dataFetcher"),ns=i(()=>{yt.clear(),B=0},"reset"),Ot="[*]",ce="start",he=Ot,ue="end",Wt="color",Jt="fill",as="bgFill",os=",";function Bt(){return new Map}i(Bt,"newClassesList");var mt=[],Ft=[],de=Be,_t=[],X=Bt(),pe=i(()=>({relations:[],states:new Map,documents:{}}),"newDoc"),bt={root:pe()},x=bt.root,tt=0,Xt=0,ls={LINE:0,DOTTED_LINE:1},cs={AGGREGATION:0,EXTENSION:1,COMPOSITION:2,DEPENDENCY:3},pt=i(t=>JSON.parse(JSON.stringify(t)),"clone"),hs=i(t=>{k.info("Setting root doc",t),_t=t},"setRootDoc"),us=i(()=>_t,"getRootDoc"),ft=i((t,e,l)=>{if(e.stmt===Rt)ft(t,e.state1,!0),ft(t,e.state2,!1);else if(e.stmt===St&&(e.id==="[*]"?(e.id=l?t.id+"_start":t.id+"_end",e.start=l):e.id=e.id.trim()),e.doc){const o=[];let r=[],h;for(h=0;h<e.doc.length;h++)if(e.doc[h].type===qt){const p=pt(e.doc[h]);p.doc=pt(r),o.push(p),r=[]}else r.push(e.doc[h]);if(o.length>0&&r.length>0){const p={stmt:St,id:Oe(),type:"divider",doc:pt(r)};o.push(pt(p)),e.doc=o}e.doc.forEach(p=>ft(e,p,!0))}},"docTranslator"),Yt=i(()=>(ft({id:"root"},{id:"root",doc:_t},!0),{id:"root",doc:_t}),"getRootDocV2"),ds=i(t=>{let e;t.doc?e=t.doc:e=t,k.info(e),ye(!0),k.info("Extract initial document:",e),e.forEach(r=>{switch(k.warn("Statement",r.stmt),r.stmt){case St:F(r.id.trim(),r.type,r.doc,r.description,r.note,r.classes,r.styles,r.textStyles);break;case Rt:_e(r.state1,r.state2,r.description);break;case Fe:be(r.id.trim(),r.classes);break;case Ye:{const h=r.id.trim().split(","),p=r.styleClass.split(",");h.forEach(f=>{let u=H(f);if(u===void 0){const S=f.trim();F(S),u=H(S)}u.styles=p.map(S=>{var _;return(_=S.replace(/;/g,""))==null?void 0:_.trim()})})}break;case Pe:Pt(r.id.trim(),r.styleClass);break}});const l=ge(),o=D().look;ns(),Z(void 0,Yt(),l,mt,Ft,!0,o,X),mt.forEach(r=>{if(Array.isArray(r.label)){if(r.description=r.label.slice(1),r.isGroup&&r.description.length>0)throw new Error("Group nodes can only have label. Remove the additional description for node ["+r.id+"]");r.label=r.label[0]}})},"extract"),F=i(function(t,e=et,l=null,o=null,r=null,h=null,p=null,f=null){const u=t==null?void 0:t.trim();if(x.states.has(u)?(x.states.get(u).doc||(x.states.get(u).doc=l),x.states.get(u).type||(x.states.get(u).type=e)):(k.info("Adding state ",u,o),x.states.set(u,{id:u,descriptions:[],type:e,doc:l,note:r,classes:[],styles:[],textStyles:[]})),o&&(k.info("Setting state description",u,o),typeof o=="string"&&wt(u,o.trim()),typeof o=="object"&&o.forEach(S=>wt(u,S.trim()))),r){const S=x.states.get(u);S.note=r,S.note.text=M.sanitizeText(S.note.text,D())}h&&(k.info("Setting state classes",u,h),(typeof h=="string"?[h]:h).forEach(S=>Pt(u,S.trim()))),p&&(k.info("Setting state styles",u,p),(typeof p=="string"?[p]:p).forEach(S=>ms(u,S.trim()))),f&&(k.info("Setting state styles",u,p),(typeof f=="string"?[f]:f).forEach(S=>_s(u,S.trim())))},"addState"),ye=i(function(t){mt=[],Ft=[],bt={root:pe()},x=bt.root,tt=0,X=Bt(),t||Ne()},"clear"),H=i(function(t){return x.states.get(t)},"getState"),ge=i(function(){return x.states},"getStates"),ps=i(function(){k.info("Documents = ",bt)},"logDocuments"),ys=i(function(){return x.relations},"getRelations");function kt(t=""){let e=t;return t===Ot&&(tt++,e=`${ce}${tt}`),e}i(kt,"startIdIfNeeded");function Tt(t="",e=et){return t===Ot?ce:e}i(Tt,"startTypeIfNeeded");function fe(t=""){let e=t;return t===he&&(tt++,e=`${ue}${tt}`),e}i(fe,"endIdIfNeeded");function Se(t="",e=et){return t===he?ue:e}i(Se,"endTypeIfNeeded");function me(t,e,l){let o=kt(t.id.trim()),r=Tt(t.id.trim(),t.type),h=kt(e.id.trim()),p=Tt(e.id.trim(),e.type);F(o,r,t.doc,t.description,t.note,t.classes,t.styles,t.textStyles),F(h,p,e.doc,e.description,e.note,e.classes,e.styles,e.textStyles),x.relations.push({id1:o,id2:h,relationTitle:M.sanitizeText(l,D())})}i(me,"addRelationObjs");var _e=i(function(t,e,l){if(typeof t=="object")me(t,e,l);else{const o=kt(t.trim()),r=Tt(t),h=fe(e.trim()),p=Se(e);F(o,r),F(h,p),x.relations.push({id1:o,id2:h,title:M.sanitizeText(l,D())})}},"addRelation"),wt=i(function(t,e){const l=x.states.get(t),o=e.startsWith(":")?e.replace(":","").trim():e;l.descriptions.push(M.sanitizeText(o,D()))},"addDescription"),gs=i(function(t){return t.substring(0,1)===":"?t.substr(2).trim():t.trim()},"cleanupLabel"),fs=i(()=>(Xt++,"divider-id-"+Xt),"getDividerId"),be=i(function(t,e=""){X.has(t)||X.set(t,{id:t,styles:[],textStyles:[]});const l=X.get(t);e!=null&&e.split(os).forEach(o=>{const r=o.replace(/([^;]*);/,"$1").trim();if(RegExp(Wt).exec(o)){const h=r.replace(Jt,as).replace(Wt,Jt);l.textStyles.push(h)}l.styles.push(r)})},"addStyleClass"),Ss=i(function(){return X},"getClasses"),Pt=i(function(t,e){t.split(",").forEach(function(l){let o=H(l);if(o===void 0){const r=l.trim();F(r),o=H(r)}o.classes.push(e)})},"setCssClass"),ms=i(function(t,e){const l=H(t);l!==void 0&&l.styles.push(e)},"setStyle"),_s=i(function(t,e){const l=H(t);l!==void 0&&l.textStyles.push(e)},"setTextStyle"),bs=i(()=>de,"getDirection"),ks=i(t=>{de=t},"setDirection"),Ts=i(t=>t&&t[0]===":"?t.substr(1).trim():t.trim(),"trimColon"),Es=i(()=>{const t=D();return{nodes:mt,edges:Ft,other:{},config:t,direction:ae(Yt())}},"getData"),Ls={getConfig:i(()=>D().state,"getConfig"),getData:Es,addState:F,clear:ye,getState:H,getStates:ge,getRelations:ys,getClasses:Ss,getDirection:bs,addRelation:_e,getDividerId:fs,setDirection:ks,cleanupLabel:gs,lineType:ls,relationType:cs,logDocuments:ps,getRootDoc:us,setRootDoc:hs,getRootDocV2:Yt,extract:ds,trimColon:Ts,getAccTitle:ve,setAccTitle:De,getAccDescription:Ce,setAccDescription:Le,addStyleClass:be,setCssClass:Pt,addDescription:wt,setDiagramTitle:Ie,getDiagramTitle:Ae},xs=i(t=>`
defs #statediagram-barbEnd {
    fill: ${t.transitionColor};
    stroke: ${t.transitionColor};
  }
g.stateGroup text {
  fill: ${t.nodeBorder};
  stroke: none;
  font-size: 10px;
}
g.stateGroup text {
  fill: ${t.textColor};
  stroke: none;
  font-size: 10px;

}
g.stateGroup .state-title {
  font-weight: bolder;
  fill: ${t.stateLabelColor};
}

g.stateGroup rect {
  fill: ${t.mainBkg};
  stroke: ${t.nodeBorder};
}

g.stateGroup line {
  stroke: ${t.lineColor};
  stroke-width: 1;
}

.transition {
  stroke: ${t.transitionColor};
  stroke-width: 1;
  fill: none;
}

.stateGroup .composit {
  fill: ${t.background};
  border-bottom: 1px
}

.stateGroup .alt-composit {
  fill: #e0e0e0;
  border-bottom: 1px
}

.state-note {
  stroke: ${t.noteBorderColor};
  fill: ${t.noteBkgColor};

  text {
    fill: ${t.noteTextColor};
    stroke: none;
    font-size: 10px;
  }
}

.stateLabel .box {
  stroke: none;
  stroke-width: 0;
  fill: ${t.mainBkg};
  opacity: 0.5;
}

.edgeLabel .label rect {
  fill: ${t.labelBackgroundColor};
  opacity: 0.5;
}
.edgeLabel {
  background-color: ${t.edgeLabelBackground};
  p {
    background-color: ${t.edgeLabelBackground};
  }
  rect {
    opacity: 0.5;
    background-color: ${t.edgeLabelBackground};
    fill: ${t.edgeLabelBackground};
  }
  text-align: center;
}
.edgeLabel .label text {
  fill: ${t.transitionLabelColor||t.tertiaryTextColor};
}
.label div .edgeLabel {
  color: ${t.transitionLabelColor||t.tertiaryTextColor};
}

.stateLabel text {
  fill: ${t.stateLabelColor};
  font-size: 10px;
  font-weight: bold;
}

.node circle.state-start {
  fill: ${t.specialStateColor};
  stroke: ${t.specialStateColor};
}

.node .fork-join {
  fill: ${t.specialStateColor};
  stroke: ${t.specialStateColor};
}

.node circle.state-end {
  fill: ${t.innerEndBackground};
  stroke: ${t.background};
  stroke-width: 1.5
}
.end-state-inner {
  fill: ${t.compositeBackground||t.background};
  // stroke: ${t.background};
  stroke-width: 1.5
}

.node rect {
  fill: ${t.stateBkg||t.mainBkg};
  stroke: ${t.stateBorder||t.nodeBorder};
  stroke-width: 1px;
}
.node polygon {
  fill: ${t.mainBkg};
  stroke: ${t.stateBorder||t.nodeBorder};;
  stroke-width: 1px;
}
#statediagram-barbEnd {
  fill: ${t.lineColor};
}

.statediagram-cluster rect {
  fill: ${t.compositeTitleBackground};
  stroke: ${t.stateBorder||t.nodeBorder};
  stroke-width: 1px;
}

.cluster-label, .nodeLabel {
  color: ${t.stateLabelColor};
  // line-height: 1;
}

.statediagram-cluster rect.outer {
  rx: 5px;
  ry: 5px;
}
.statediagram-state .divider {
  stroke: ${t.stateBorder||t.nodeBorder};
}

.statediagram-state .title-state {
  rx: 5px;
  ry: 5px;
}
.statediagram-cluster.statediagram-cluster .inner {
  fill: ${t.compositeBackground||t.background};
}
.statediagram-cluster.statediagram-cluster-alt .inner {
  fill: ${t.altBackground?t.altBackground:"#efefef"};
}

.statediagram-cluster .inner {
  rx:0;
  ry:0;
}

.statediagram-state rect.basic {
  rx: 5px;
  ry: 5px;
}
.statediagram-state rect.divider {
  stroke-dasharray: 10,10;
  fill: ${t.altBackground?t.altBackground:"#efefef"};
}

.note-edge {
  stroke-dasharray: 5;
}

.statediagram-note rect {
  fill: ${t.noteBkgColor};
  stroke: ${t.noteBorderColor};
  stroke-width: 1px;
  rx: 0;
  ry: 0;
}
.statediagram-note rect {
  fill: ${t.noteBkgColor};
  stroke: ${t.noteBorderColor};
  stroke-width: 1px;
  rx: 0;
  ry: 0;
}

.statediagram-note text {
  fill: ${t.noteTextColor};
}

.statediagram-note .nodeLabel {
  color: ${t.noteTextColor};
}
.statediagram .edgeLabel {
  color: red; // ${t.noteTextColor};
}

#dependencyStart, #dependencyEnd {
  fill: ${t.lineColor};
  stroke: ${t.lineColor};
  stroke-width: 1;
}

.statediagramTitleText {
  text-anchor: middle;
  font-size: 18px;
  fill: ${t.textColor};
}
`,"getStyles"),Is=xs;export{Ds as A,Is as I,Cs as L,Ls as R};
