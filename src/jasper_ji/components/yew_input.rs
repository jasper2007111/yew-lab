use gloo_console::log;
use std::collections::HashMap;
use std::fmt::Debug;
use yew::prelude::*;
use yew::virtual_dom::{VNode, VTag};

pub enum YewInputMsg {}

pub struct YewInput {
    input_ref: NodeRef,
    password_visible: bool,
    // 缓存查找的
    solt_map: HashMap<String, bool>,
    props: YewInputProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewInputProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or("text".to_string())]
    pub input_type: String,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub tabindex: String,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub show_password: bool,

    #[prop_or_default]
    show_clear: bool,

    // 前缀
    #[prop_or_default]
    pub prefix_icon: String,

    // 后缀
    #[prop_or_default]
    pub suffix_icon: String,

    #[prop_or_default]
    pub children: Children,
}

impl Component for YewInput {
    type Message = YewInputMsg;
    type Properties = YewInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            solt_map: HashMap::default(),
            input_ref: NodeRef::default(),
            password_visible: false,
            props: ctx.props().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = self.get_root_div_classes();
        html! {
            <div class={classes!(classes.clone())}>
                if self.props.input_type == "textarea" {
                    // TODO 暂未实现
                } else {
                    {self.get_input_node()}
                }
            </div>
        }
    }
}

impl YewInput {
    pub fn get_root_div_classes(&self) -> Vec<String> {
        let mut classes: Vec<_> = vec![];
        if self.props.input_type == "textarea" {
            classes.push("el-textarea".to_string());
        } else {
            classes.push("el-input".to_string());
        }

        if !self.props.size.is_empty() {
            classes.push(format!("el-input--{}", self.props.size));
        }
        if self.is_input_disabled() {
            classes.push("is-disabled".to_string());
        }
        if self.is_input_exceed() {
            classes.push("is-exceed".to_string());
        }

        let has_prepend = self.has_solt("prepend".to_string());
        let has_append = self.has_solt("append".to_string());
        if has_prepend || has_append {
            classes.push("el-input-group".to_string());
            if has_append {
                classes.push("el-input-group--append".to_string());
            }
            if has_prepend {
                classes.push("el-input-group--prepend".to_string());
            }
        }

        let has_prefix = self.has_solt("prefix".to_string());
        if has_prefix || !self.props.prefix_icon.is_empty() {
            classes.push("el-input--prefix".to_string());
        }

        // TODO 以下未实现全
        // 'el-input--suffix': $slots.suffix || suffixIcon || clearable || showPassword
        let has_suffix = self.has_solt("suffix".to_string());
        if has_suffix || !self.props.suffix_icon.is_empty() {
            classes.push("el-input--suffix".to_string());
        }

        classes
    }

    pub fn is_input_disabled(&self) -> bool {
        return self.props.disabled;
    }

    pub fn is_input_exceed(&self) -> bool {
        // TODO 暂未实现
        false
    }

    pub fn get_suffix_visible(&self) -> bool {
        // this.isWordLimitVisible ||
        // (this.validateState && this.needStatusIcon);

        let has_suffix = self.has_solt("suffix".to_string());
        if has_suffix {
            return true;
        }

        if !self.props.suffix_icon.is_empty() {
            return true;
        }

        if self.props.show_clear {
            return true;
        }

        if self.props.show_password {
            return true;
        }

        return false;
    }

    pub fn show_pwd_visible(&self) -> bool {
        // return this.showPassword &&
        // !this.inputDisabled &&
        // !this.readonly &&
        // (!!this.nativeInputValue || this.focused);

        return self.props.show_password;
    }

    pub fn is_word_limit_visible(&self) -> bool {
        // return this.showWordLimit &&
        //   this.$attrs.maxlength &&
        //   (this.type === 'text' || this.type === 'textarea') &&
        //   !this.inputDisabled &&
        //   !this.readonly &&
        //   !this.showPassword;
        return false;
    }

    pub fn get_solt(&self, solt_name: String) -> Option<VNode> {
        for i in self.props.children.clone().into_iter() {
            match i {
                VNode::VTag(ref vtag) => {
                    match vtag.attributes {
                        yew::virtual_dom::Attributes::Static(vev) => {
                            for g in vev {
                                if g.0 == "solt" {
                                    // log!(format!("{:?}", g.1));
                                    if g.1 == solt_name {
                                        return Some(i);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn has_solt(&self, name: String) -> bool {
        // TODO 此处原本打算缓存一下，但发现这个最终会在view的方法里调用，而那个是不可修改的self。
        // 这个也让我知道了，Rust的一些使用上的问题，安全的代价可能比想象的高。
        // let clone_name = name.clone();
        // let c = self.solt_map.get(&clone_name.clone());
        // if c.is_some() {
        //     return *c.unwrap();
        // }
        match self.get_solt(name) {
            Some(_) => {
                // self.solt_map.insert(clone_name.clone(), true);
                return true;
            }
            None => {
                // self.solt_map.insert(clone_name.clone(), false);
                return false;
            }
        }
    }

    pub fn get_input_node(&self) -> VNode {
        let input_type;
        if self.props.show_password {
            if self.password_visible {
                input_type = "text".to_string()
            } else {
                input_type = "password".to_string()
            }
        } else {
            input_type = self.props.input_type.clone()
        };
        let has_prepend = self.has_solt("prepend".to_string());
        let has_append = self.has_solt("append".to_string());
        let has_prefix = self.has_solt("prefix".to_string());
        let has_suffix = self.has_solt("suffix".to_string());
        return html!(
            <>
                // 前置元素
                if has_prepend {
                    <div class="el-input-group__prepend">
                        {self.get_solt("prepend".to_string()).unwrap().clone()}
                    </div>
                }
                <input
                    tabindex={self.props.tabindex.clone()}
                    disabled={self.is_input_disabled()}
                    type={input_type}
                    class="el-input__inner"
                    readonly = {self.props.readonly}
                    ref = {&self.input_ref}
                    autocomplete="off"
                    placeholder="请选择日期"
                />
                // 前置内容
                if has_prefix || !self.props.prefix_icon.is_empty() {
                    <span class="el-input__prefix">
                        if has_prefix {
                            {self.get_solt("prefix".to_string()).unwrap().clone()}
                        }
                        if !self.props.prefix_icon.is_empty() {
                            <i class={format!("el-input__icon {}",self.props.prefix_icon.clone())}></i>
                        }
                    </span>
                }

                // 后置内容
                if self.get_suffix_visible() {
                    <span class="el-input__suffix">
                        <span class="el-input__suffix-inner">
                            if self.props.show_clear || self.show_pwd_visible() || self.is_word_limit_visible() {
                                // TODO 暂未实现
                                // <i v-if="showClear"
                                // class="el-input__icon el-icon-circle-close el-input__clear"
                                // @mousedown.prevent
                                // @click="clear"
                                // ></i>
                                // <i v-if="showPwdVisible"
                                // class="el-input__icon el-icon-view el-input__clear"
                                // @click="handlePasswordVisible"
                                // ></i>
                                // <span v-if="isWordLimitVisible" class="el-input__count">
                                // <span class="el-input__count-inner">
                                //     {{ textLength }}/{{ upperLimit }}
                                // </span>
                                // </span>
                            } else {
                                if has_suffix {
                                    {self.get_solt("suffix".to_string()).unwrap().clone()}
                                }
                                if !self.props.suffix_icon.is_empty() {
                                    <i class={format!("el-input__icon {}", self.props.suffix_icon)}/>
                                }
                            }
                        </span>
                    </span>
                }
                if has_append {
                    <div class="el-input-group__append">
                        {self.get_solt("append".to_string()).unwrap().clone()}
                    </div>
                }
            </>
        );
    }
}
