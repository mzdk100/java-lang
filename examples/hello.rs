fn main() -> anyhow::Result<()> {
    let input = r#"
        package com.test;

        import java.lang.System;

        public class HelloWorld {
            /**
             * 主函数。
             */
            public static void main(String[] args) {
                int a=0_1;
                int b=a+3;
                b--;
                b++;
                boolean bb = true;
                char c = 'a';
                float f=(float)1.0;
                float f2 = 1_2.1f;
                int h = 0x01f;
                int o = 077;
                int bin = 0b0110;
                Object obj = null;  // 空值
                // 测试注释
                System.out.println("Hello, World!");
                /*
                123456
                abcdef
                */
            }
        }
    "#;

    let (remaining, tokens) = javalang::tokenize(input)?;
    print!("{}", remaining);
    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}
