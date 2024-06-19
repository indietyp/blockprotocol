import { SvgIcon, SvgIconProps } from "@mui/material";
import { FunctionComponent } from "react";

export const OpenAiIcon: FunctionComponent<SvgIconProps> = ({
  sx,
  ...props
}) => {
  return (
    <SvgIcon
      {...props}
      width="74"
      height="19"
      viewBox="0 0 74 19"
      sx={[{ color: "#64778C" }, ...(Array.isArray(sx) ? sx : [sx])]}
    >
      <title>Open AI</title>
      <path
        d="M43.4394 7.50364V7.50969C43.4027 7.50969 43.3658 7.51574 43.3292 7.51574C43.2925 7.51574 43.2556 7.50969 43.2189 7.50969C41.0323 7.50969 39.6787 8.86911 39.6787 11.0512V12.124C39.6787 14.2269 41.0508 15.5314 43.2495 15.5314C43.2945 15.5326 43.3396 15.5306 43.3843 15.5254C43.4149 15.5254 43.4394 15.5314 43.47 15.5314C44.946 15.5314 45.9752 14.9951 46.6304 13.8856L45.3258 13.1357C44.8909 13.7757 44.309 14.2573 43.4761 14.2573C42.3614 14.2573 41.6938 13.5746 41.6938 12.4225V12.1178H46.8633V10.8499C46.8633 8.81403 45.522 7.50364 43.4394 7.50364ZM43.3292 8.75334C44.3459 8.80215 44.9584 9.47258 44.9584 10.5758V10.8806H41.6878V10.7037C41.6878 9.48467 42.2696 8.80215 43.3292 8.75334ZM35.2995 7.50969C34.3257 7.50969 33.4867 7.91207 33.0457 8.5825L32.9354 8.75313V7.69241H31.0857V17.9879H33.0272V14.3917L33.1375 14.5563C33.5539 15.1718 34.3687 15.5375 35.318 15.5375H35.367H35.4098C37.0084 15.5375 38.6192 14.5012 38.6192 12.1789V10.8745C38.6192 9.20432 37.6269 7.51595 35.3974 7.51595L35.3852 7.50991H35.3425L35.2995 7.50969ZM34.8464 8.96047C35.9734 8.97883 36.6716 9.74062 36.6716 10.9599V12.0813C36.6716 13.3003 35.9671 14.0563 34.8279 14.0807C33.7684 14.0623 33.0272 13.2577 33.0272 12.1117V10.9596C33.0272 9.80153 33.7744 8.98488 34.8464 8.96047ZM60.0074 4.96796L56.2713 15.3548H58.3721L59.0887 13.1299H63.1678V13.1543L63.8844 15.3608H65.9853L62.243 4.974H62.0225L62.0164 4.96796H60.0074ZM61.1223 6.75395L62.684 11.661H59.548L61.1223 6.75395ZM73.5005 6.43084V4.96796H67.0938V6.43084H69.3416V13.8796H67.0938V15.3425H73.5005V13.8796H71.2528V6.43084H73.5005ZM52.4186 7.50991H52.3634H52.3328C51.2488 7.50991 50.477 7.87557 50.0972 8.57666L49.9809 8.79006V7.69285H48.1311V15.3487H50.0727V10.7892C50.0727 9.71643 50.6545 9.10086 51.6589 9.08251C52.6206 9.10086 53.1719 9.70433 53.1719 10.7467V15.3487H55.1134V10.4173C55.1134 8.59481 54.109 7.50969 52.4246 7.50969L52.4186 7.50991ZM24.8441 4.75456C21.9837 4.75456 20.2014 6.52846 20.2014 9.38726V10.9294C20.2014 13.7882 21.9776 15.5619 24.8441 15.5619H24.8871H24.9299C27.7903 15.5619 29.5725 13.7882 29.5725 10.9294V9.38726C29.5725 6.52846 27.7903 4.75456 24.9299 4.75456H24.8871H24.8441ZM24.8871 6.30297C26.5899 6.32133 27.5698 7.40623 27.5698 9.28359V11.0391C27.5698 12.9165 26.5899 14.0016 24.8871 14.0197C23.1843 14.0014 22.2042 12.9165 22.2042 11.0391V9.28359C22.2042 7.40623 23.1843 6.32111 24.8871 6.30297ZM7.9762 0.500052C5.99776 0.500052 4.24004 1.76789 3.62737 3.6392C2.99862 3.76819 2.40469 4.02896 1.88509 4.40417C1.3655 4.77938 0.93216 5.26042 0.613917 5.81527C-0.37834 7.522 -0.151761 9.66762 1.17754 11.1305C0.76714 12.3558 0.907992 13.6966 1.56342 14.8062C2.5496 16.5189 4.5339 17.3967 6.47566 16.9883C6.9008 17.4658 7.42336 17.8476 8.00848 18.1083C8.5936 18.369 9.22784 18.5025 9.86891 18.5C11.8473 18.5 13.6051 17.2321 14.2177 15.3608C15.4917 15.0986 16.5881 14.3061 17.2251 13.1847C18.2234 11.478 17.9969 9.3324 16.6678 7.86952V7.86348C16.8705 7.25782 16.941 6.61607 16.8744 5.98114C16.8079 5.34621 16.6059 4.73274 16.2819 4.18176C15.2955 2.47504 13.311 1.59705 11.3755 2.00548C10.9484 1.52921 10.4245 1.1487 9.83844 0.889167C9.2524 0.629633 8.61763 0.496998 7.9762 0.500052ZM7.9762 1.67048L7.97012 1.67653C8.7664 1.67653 9.53208 1.95084 10.1445 2.45689C10.12 2.46899 10.071 2.49944 10.0343 2.5178L6.4329 4.58415C6.24908 4.68782 6.13883 4.88286 6.13883 5.09625V9.94818L4.58924 9.05832V5.04744C4.58889 4.15277 4.94541 3.29456 5.58048 2.66133C6.21555 2.0281 7.07722 1.67186 7.9762 1.67048ZM12.3146 3.08304C12.9114 3.0819 13.498 3.23758 14.0149 3.53434C14.5319 3.8311 14.961 4.25843 15.2588 4.77314C15.6508 5.45587 15.7977 6.25437 15.6629 7.02847C15.6384 7.01011 15.5896 6.98592 15.5588 6.96756L11.9574 4.89495C11.8668 4.84396 11.7645 4.81716 11.6604 4.81716C11.5563 4.81716 11.454 4.84396 11.3634 4.89495L7.14324 7.32092V5.54097L10.6283 3.53553C11.1407 3.23988 11.7223 3.08396 12.3146 3.08304ZM3.40708 4.92541V9.18618C3.40708 9.39957 3.51733 9.58856 3.70116 9.69829L7.91499 12.1182L6.35911 13.0141L2.88014 11.0147C2.10266 10.5663 1.53574 9.82921 1.30381 8.96523C1.07188 8.10124 1.19389 7.18097 1.64307 6.40643C2.03931 5.7229 2.66337 5.1991 3.40708 4.92541ZM11.4797 5.97985L14.9648 7.97925C16.5879 8.91188 17.1389 10.9722 16.2018 12.5875L16.2079 12.5936C15.8099 13.2763 15.185 13.8005 14.4439 14.0688V9.80779C14.4439 9.59439 14.3336 9.39936 14.1498 9.2959L9.92989 6.86972L11.4797 5.97985ZM8.9194 7.449L10.6958 8.47299V10.5149L8.9194 11.5389L7.14324 10.5149V8.47299L8.9194 7.449ZM11.7124 9.05832L13.2619 9.94818V13.953C13.2619 15.8183 11.743 17.33 9.87498 17.33V17.3239C9.08478 17.3239 8.31303 17.0496 7.70665 16.5438C7.73117 16.5317 7.7863 16.501 7.8169 16.4826L11.4183 14.4163C11.6021 14.3126 11.7184 14.1176 11.7121 13.9042L11.7124 9.05832ZM10.7016 11.6793V13.459L7.21659 15.4584C5.59343 16.385 3.52319 15.8364 2.58606 14.2271H2.59214C2.19411 13.5504 2.05304 12.7459 2.18781 11.9718C2.21234 11.9901 2.26139 12.0143 2.29199 12.0327L5.89337 14.1053C5.98397 14.1563 6.08629 14.1831 6.19037 14.1831C6.29446 14.1831 6.39678 14.1563 6.48738 14.1053L10.7016 11.6793Z"
        fill="currentColor"
      />
    </SvgIcon>
  );
};
